//! Milestone-4 fragment compilation: math blocks compiled by the real Typst
//! compiler and rasterized to bitmaps the view draws in place.
//!
//! Quarantine rule: this module and `app/documents/typst.rs` are the only
//! places allowed to depend on `typst-*` crates.

use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash as _, Hasher as _};
use std::path::PathBuf;
use std::sync::{Arc, LazyLock};

use typst::diag::{FileError, FileResult};
use typst::foundations::{Bytes, Datetime, Duration};
use typst::text::{Font, FontBook};
use typst::utils::{LazyHash, Scalar};
use typst::{Library, LibraryExt as _, World};
use typst_layout::PagedDocument;
use typst_render::RenderOptions;
use typst_syntax::{FileId, Source};

use crate::core::preview_renderer::RenderedFragment;

/// Rasterization density: 2 device pixels per logical pixel, and 96/72
/// logical pixels per point — crisp on hidpi, fine on lodpi.
const PIXELS_PER_POINT: f64 = 2.0 * 96.0 / 72.0;

/// The math text size in points, matching the editor's 14px base font
/// (1px = 0.75pt).
const MATH_SIZE_PT: f32 = 10.5;

/// The design's code/math gold.
const MATH_COLOR: &str = "#E9C46A";

/// Embedded default fonts (`typst-assets`), loaded once per process — math
/// rendering must not depend on system fonts.
static FONTS: LazyLock<Vec<Font>> = LazyLock::new(|| {
    typst_assets::fonts()
        .flat_map(|data| Font::iter(Bytes::new(data)))
        .collect()
});

/// Compiles standalone math fragments and caches the resulting bitmaps by
/// source-text hash, so unchanged math costs a hash lookup per frame and
/// only edited blocks recompile. Failed compiles cache as `None` — the
/// caller falls back to styled text, and broken markup is not an error.
pub struct FragmentCompiler {
    shared: Arc<SharedWorld>,
    cache: RefCell<HashMap<u64, Option<Arc<RenderedFragment>>>>,
}

impl FragmentCompiler {
    pub fn new() -> Self {
        let fonts = FONTS.clone();
        Self {
            shared: Arc::new(SharedWorld {
                library: LazyHash::new(Library::builder().build()),
                book: LazyHash::new(FontBook::from_fonts(&fonts)),
                fonts,
            }),
            cache: RefCell::new(HashMap::new()),
        }
    }

    /// Compiles the body of a block math equation (the text between the `$ `
    /// delimiters) to a bitmap, typeset in display style. Returns the cached
    /// fragment when the same body was compiled before; `None` when Typst
    /// rejects the markup.
    pub fn render_math(&self, math_body: &str) -> Option<Arc<RenderedFragment>> {
        self.render_cached(math_body, MathStyle::Display)
    }

    /// Compiles the body of an inline math equation, typeset in inline style
    /// (shrunk limits, text-height fractions).
    pub fn render_inline_math(&self, math_body: &str) -> Option<Arc<RenderedFragment>> {
        self.render_cached(math_body, MathStyle::Inline)
    }

    fn render_cached(&self, math_body: &str, style: MathStyle) -> Option<Arc<RenderedFragment>> {
        let mut hasher = DefaultHasher::new();
        (math_body, style == MathStyle::Inline).hash(&mut hasher);
        let key = hasher.finish();

        if let Some(cached) = self.cache.borrow().get(&key) {
            return cached.clone();
        }
        let fragment = self.compile(math_body, style).map(Arc::new);
        self.cache.borrow_mut().insert(key, fragment.clone());
        fragment
    }

    fn compile(&self, math_body: &str, style: MathStyle) -> Option<RenderedFragment> {
        // In Typst, whitespace inside the delimiters selects the style:
        // `$ x $` typesets as display math, `$x$` as inline math.
        let equation = match style {
            MathStyle::Display => format!("$ {math_body} $"),
            MathStyle::Inline => format!("${math_body}$"),
        };
        let text = format!(
            "#set page(width: auto, height: auto, margin: 0pt, fill: none)\n\
             #set text(size: {MATH_SIZE_PT}pt, fill: rgb(\"{MATH_COLOR}\"))\n\
             {equation}\n"
        );
        let world = FragmentWorld {
            shared: Arc::clone(&self.shared),
            main: Source::detached(text),
        };
        let document = typst::compile::<PagedDocument>(&world).output.ok()?;
        // Bound the compiler's memoization cache across fragment compiles.
        comemo::evict(10);
        let page = document.pages().first()?;

        let options = RenderOptions {
            pixel_per_pt: Scalar::new(PIXELS_PER_POINT),
            ..RenderOptions::default()
        };
        let pixmap = typst_render::render(page, &options);
        let (width, height) = (pixmap.width(), pixmap.height());
        if width == 0 || height == 0 {
            return None;
        }

        // tiny-skia yields premultiplied RGBA; GPUI wants straight BGRA.
        let mut bgra = Vec::with_capacity(width as usize * height as usize * 4);
        for pixel in pixmap.pixels() {
            let color = pixel.demultiply();
            bgra.extend_from_slice(&[color.blue(), color.green(), color.red(), color.alpha()]);
        }

        Some(RenderedFragment {
            width,
            height,
            logical_width: width as f32 / 2.0,
            logical_height: height as f32 / 2.0,
            bgra,
        })
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum MathStyle {
    Display,
    Inline,
}

impl Default for FragmentCompiler {
    fn default() -> Self {
        Self::new()
    }
}

/// The compile environment shared across fragment compiles: the standard
/// library and the embedded font set. Each compile pairs it with a fresh
/// single-file main source in [`FragmentWorld`].
struct SharedWorld {
    library: LazyHash<Library>,
    book: LazyHash<FontBook>,
    fonts: Vec<Font>,
}

struct FragmentWorld {
    shared: Arc<SharedWorld>,
    main: Source,
}

impl World for FragmentWorld {
    fn library(&self) -> &LazyHash<Library> {
        &self.shared.library
    }

    fn book(&self) -> &LazyHash<FontBook> {
        &self.shared.book
    }

    fn main(&self) -> FileId {
        self.main.id()
    }

    fn source(&self, id: FileId) -> FileResult<Source> {
        if id == self.main.id() {
            Ok(self.main.clone())
        } else {
            Err(FileError::NotFound(PathBuf::from(
                id.vpath().get_without_slash(),
            )))
        }
    }

    fn file(&self, id: FileId) -> FileResult<Bytes> {
        Err(FileError::NotFound(PathBuf::from(
            id.vpath().get_without_slash(),
        )))
    }

    fn font(&self, index: usize) -> Option<Font> {
        self.shared.fonts.get(index).cloned()
    }

    fn today(&self, _offset: Option<Duration>) -> Option<Datetime> {
        None
    }
}
