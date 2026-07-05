use crate::app::documents::typst::TypstDocument;
use crate::core::editable_buffer::EditableBuffer;
use crate::core::position::{Position, Range};
use crate::core::preview_renderer::{BlockKind, PreviewRenderer, SpanKind};

// --- EditableBuffer half ---------------------------------------------------

#[test]
fn edits_keep_text_and_syntax_tree_in_sync() {
    let mut document = TypstDocument::new("= Heading\n\nSome *prose*.");
    assert!(format!("{:?}", document.syntax()).contains("Heading"));

    document.replace(Range::new(Position(2), Position(9)), "Title");
    assert_eq!(document.text(), "= Title\n\nSome *prose*.");
    assert!(format!("{:?}", document.syntax()).contains("Title"));
}

#[test]
fn clamps_out_of_range_and_mid_character_edits() {
    let mut document = TypstDocument::new("a💝b");
    document.replace(Range::caret(Position(99)), "c");
    assert_eq!(document.text(), "a💝bc");
    document.replace(Range::new(Position(2), Position(5)), "");
    assert_eq!(document.text(), "abc");
}

// --- PreviewRenderer half --------------------------------------------------

fn preview(text: &str) -> Vec<crate::core::preview_renderer::PreviewBlock> {
    // A cursor far past the end keeps every block styled.
    TypstDocument::new(text)
        .render_preview(&[Position(usize::MAX)])
        .blocks
}

#[test]
fn heading_hides_markers_and_carries_level() {
    let blocks = preview("== Convergence");
    assert_eq!(blocks.len(), 1);
    assert_eq!(blocks[0].kind(), BlockKind::Heading(2));
    assert_eq!(blocks[0].display_text(), "Convergence");
    // The block's source range still covers the markers.
    assert_eq!(
        blocks[0].source_range,
        Range::new(Position(0), Position(14))
    );
    // Display offset 0 maps back past the hidden "== ".
    assert_eq!(blocks[0].offset_map.display_to_source(0), Position(3));
    assert_eq!(blocks[0].offset_map.source_to_display(Position(3)), 0);
}

#[test]
fn strong_and_emph_hide_markers_and_style_spans() {
    let blocks = preview("A *metric space* is _nice_.");
    assert_eq!(blocks.len(), 1);
    assert_eq!(blocks[0].display_text(), "A metric space is nice.");
    let strong = blocks[0]
        .spans()
        .iter()
        .find(|span| span.kind == SpanKind::Strong)
        .expect("strong span");
    assert_eq!(
        &blocks[0].display_text()[strong.range.clone()],
        "metric space"
    );
    let emph = blocks[0]
        .spans()
        .iter()
        .find(|span| span.kind == SpanKind::Emphasis)
        .expect("emphasis span");
    assert_eq!(&blocks[0].display_text()[emph.range.clone()], "nice");
}

#[test]
fn inline_math_hides_dollars() {
    let blocks = preview("Let $x in M$ hold.");
    assert_eq!(blocks[0].display_text(), "Let x in M hold.");
    let math = blocks[0]
        .spans()
        .iter()
        .find(|span| span.kind == SpanKind::InlineMath)
        .expect("math span");
    assert_eq!(&blocks[0].display_text()[math.range.clone()], "x in M");
}

#[test]
fn paragraphs_split_on_blank_lines() {
    let blocks = preview("one\n\ntwo");
    assert_eq!(blocks.len(), 2);
    assert_eq!(blocks[0].display_text(), "one");
    assert_eq!(blocks[1].display_text(), "two");
}

#[test]
fn block_math_and_code_get_their_own_blocks() {
    let blocks = preview("$ x + y $\n\n```rust\nfn main() {}\n```");
    // Valid block math compiles to a bitmap fragment (milestone 4); the
    // styled text and offset map stay behind it for hit testing.
    assert_eq!(blocks[0].kind(), BlockKind::Rendered);
    assert!(blocks[0].rendered_fragment().is_some());
    assert_eq!(blocks[0].display_text(), "x + y");
    assert_eq!(blocks[1].kind(), BlockKind::CodeBlock);
    assert_eq!(blocks[1].display_text(), "fn main() {}");
}

#[test]
fn math_that_does_not_compile_falls_back_to_styled_text() {
    let blocks = preview("$ nosuchfunction() $");
    assert_eq!(blocks[0].kind(), BlockKind::MathBlock);
    assert!(blocks[0].rendered_fragment().is_none());
}

#[test]
fn math_with_the_cursor_inside_stays_raw() {
    let document = TypstDocument::new("$ x + y $");
    let output = document.render_preview(&[Position(3)]);
    assert_eq!(output.blocks[0].kind(), BlockKind::Raw);
    assert!(output.blocks[0].rendered_fragment().is_none());
    assert_eq!(output.blocks[0].display_text(), "$ x + y $");
}

#[test]
fn list_items_show_display_only_markers() {
    let blocks = preview("- alpha\n- beta");
    assert_eq!(blocks.len(), 2);
    assert_eq!(blocks[0].kind(), BlockKind::ListItem);
    assert_eq!(blocks[0].display_text(), "•  alpha");
    // Clicking the display-only marker resolves to the item's text start.
    let source = blocks[0].offset_map.display_to_source(0);
    assert_eq!(source, Position(2));
}

#[test]
fn enum_items_are_numbered() {
    let blocks = preview("+ first\n+ second");
    assert_eq!(blocks[0].display_text(), "1. first");
    assert_eq!(blocks[1].display_text(), "2. second");
}

#[test]
fn cursor_inside_a_block_reveals_raw_markup() {
    let text = "= Title\n\nbody";
    let document = TypstDocument::new(text);

    let output = document.render_preview(&[Position(3)]);
    assert_eq!(output.blocks[0].kind(), BlockKind::Raw);
    assert_eq!(output.blocks[0].display_text(), "= Title");
    // Identity mapping inside the raw block.
    assert_eq!(
        output.blocks[0].offset_map.source_to_display(Position(3)),
        3
    );
    // The other block stays styled.
    assert_eq!(output.blocks[1].kind(), BlockKind::Paragraph);
}

#[test]
fn multi_line_paragraphs_keep_their_line_breaks() {
    let blocks = preview("first line\nsecond line");
    assert_eq!(blocks.len(), 1);
    assert_eq!(blocks[0].display_text(), "first line\nsecond line");
}

#[test]
fn preview_survives_incremental_edits() {
    let mut document = TypstDocument::new("= Title\n\n*bold*");
    document.replace(Range::caret(Position(7)), "!");
    let output = document.render_preview(&[Position(usize::MAX)]);
    assert_eq!(output.blocks[0].display_text(), "Title!");
}
