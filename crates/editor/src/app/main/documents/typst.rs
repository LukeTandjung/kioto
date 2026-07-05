use typst_syntax::ast::{self, AstNode as _};
use typst_syntax::{Source, SyntaxNode};

use crate::app::documents::fragments::FragmentCompiler;
use crate::core::editable_buffer::EditableBuffer;
use crate::core::position::{Position, Range};
use crate::core::preview_renderer::{
    BlockKind, OffsetMap, PreviewBlock, PreviewOutput, PreviewRenderer, SpanKind, StyleSpan,
};

/// The Typst document model. Wraps `typst_syntax::Source`, which owns the
/// text, the lossless syntax tree, and incremental reparsing — every edit
/// keeps the AST in sync with no separate parse step. Owns both the
/// `EditableBuffer` and `PreviewRenderer` halves over that shared parse
/// state (the buffer/renderer pairing decision).
///
/// Quarantine rule: this module (plus the milestone-4 fragment module) is
/// the only place allowed to depend on `typst-*` crates.
pub struct TypstDocument {
    source: Source,
    fragments: FragmentCompiler,
}

impl TypstDocument {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            source: Source::detached(text),
            fragments: FragmentCompiler::new(),
        }
    }

    #[cfg(test)]
    pub fn syntax(&self) -> &SyntaxNode {
        self.source.root()
    }

    fn node_range(&self, node: &SyntaxNode) -> Range {
        let range = self
            .source
            .find(node.span())
            .map(|linked| linked.range())
            .unwrap_or(0..0);
        Range::new(Position(range.start), Position(range.end))
    }
}

impl EditableBuffer for TypstDocument {
    fn text(&self) -> &str {
        self.source.text()
    }

    fn replace(&mut self, range: Range, new_text: &str) {
        let clamped = range.byte_range_in(self.source.text());
        self.source.edit(clamped, new_text);
    }
}

impl PreviewRenderer for TypstDocument {
    fn render_preview(&self, cursor_positions: &[Position]) -> PreviewOutput {
        let Some(markup) = self.source.root().cast::<ast::Markup>() else {
            return PreviewOutput::default();
        };

        let mut blocks = Vec::new();
        let mut paragraph: Option<BlockSink> = None;
        let mut enum_counter = 0usize;

        for expr in markup.exprs() {
            let node = expr.to_untyped();
            // Whitespace between blocks neither starts a paragraph nor
            // resets the enumeration counter.
            if paragraph.is_none() && matches!(expr, ast::Expr::Space(_)) {
                continue;
            }
            if !matches!(expr, ast::Expr::EnumItem(_) | ast::Expr::Space(_)) {
                enum_counter = 0;
            }
            match expr {
                ast::Expr::Parbreak(_) => {
                    flush(&mut paragraph, &mut blocks);
                }
                ast::Expr::Heading(heading) => {
                    flush(&mut paragraph, &mut blocks);
                    let mut sink = BlockSink::new(self);
                    sink.include(node);
                    sink.emit_markup(heading.body(), SpanKind::Plain);
                    blocks.extend(sink.finish(BlockKind::Heading(
                        heading.depth().get().min(u8::MAX as usize) as u8,
                    )));
                }
                ast::Expr::ListItem(item) => {
                    flush(&mut paragraph, &mut blocks);
                    let mut sink = BlockSink::new(self);
                    sink.include(node);
                    sink.push_display_only("•  ");
                    sink.emit_markup(item.body(), SpanKind::Plain);
                    blocks.extend(sink.finish(BlockKind::ListItem));
                }
                ast::Expr::EnumItem(item) => {
                    flush(&mut paragraph, &mut blocks);
                    enum_counter += 1;
                    let mut sink = BlockSink::new(self);
                    sink.include(node);
                    sink.push_display_only(&format!("{enum_counter}. "));
                    sink.emit_markup(item.body(), SpanKind::Plain);
                    blocks.extend(sink.finish(BlockKind::ListItem));
                }
                ast::Expr::Raw(raw) if raw.block() => {
                    flush(&mut paragraph, &mut blocks);
                    let mut sink = BlockSink::new(self);
                    sink.include(node);
                    sink.emit_raw_inner(&raw);
                    blocks.extend(sink.finish(BlockKind::CodeBlock));
                }
                ast::Expr::Equation(equation) if equation.block() => {
                    flush(&mut paragraph, &mut blocks);
                    let mut sink = BlockSink::new(self);
                    sink.include(node);
                    sink.push_verbatim(equation.body().to_untyped(), SpanKind::Plain);
                    // Compiled math draws as a bitmap; the styled text block
                    // is the fallback when the markup does not compile.
                    blocks.extend(sink.finish(BlockKind::MathBlock).map(|block| {
                        if let Some(fragment) = self.fragments.render_math(block.display_text()) {
                            block.into_rendered(fragment)
                        } else {
                            block
                        }
                    }));
                }
                _ => {
                    let sink = paragraph.get_or_insert_with(|| BlockSink::new(self));
                    sink.include(node);
                    sink.emit_inline(&expr, SpanKind::Plain);
                }
            }
        }
        flush(&mut paragraph, &mut blocks);

        // The live-preview rule: any block containing a cursor renders as
        // raw markup so the user edits exactly what is in the source.
        for block in &mut blocks {
            let contains_cursor = cursor_positions.iter().any(|cursor| {
                block.source_range.start <= *cursor && *cursor <= block.source_range.end
            });
            if contains_cursor {
                *block = PreviewBlock::raw(
                    block.source_range,
                    block.source_range.slice(self.source.text()).to_string(),
                    OffsetMap::identity(&block.source_range),
                );
            }
        }

        PreviewOutput { blocks }
    }
}

fn flush(paragraph: &mut Option<BlockSink>, blocks: &mut Vec<PreviewBlock>) {
    if let Some(sink) = paragraph.take() {
        blocks.extend(sink.finish(BlockKind::Paragraph));
    }
}

/// Accumulates one block: display text, style spans, and the offset map,
/// all kept in lockstep.
struct BlockSink<'a> {
    document: &'a TypstDocument,
    display: String,
    spans: Vec<StyleSpan>,
    map: OffsetMap,
    range: Option<Range>,
}

impl<'a> BlockSink<'a> {
    fn new(document: &'a TypstDocument) -> Self {
        Self {
            document,
            display: String::new(),
            spans: Vec::new(),
            map: OffsetMap::default(),
            range: None,
        }
    }

    /// Expands the block's source range to cover `node` (markers included,
    /// even when their text is hidden from display).
    fn include(&mut self, node: &SyntaxNode) {
        let range = self.document.node_range(node);
        self.range = Some(match self.range {
            None => range,
            Some(current) => Range::new(current.start.min(range.start), current.end.max(range.end)),
        });
    }

    /// Emits `node`'s source text verbatim: mapped 1:1 and styled `kind`.
    fn push_verbatim(&mut self, node: &SyntaxNode, kind: SpanKind) {
        let source_range = self.document.node_range(node);
        let range = source_range.byte_range_in(self.document.source.text());
        if range.is_empty() {
            return;
        }
        let text = &self.document.source.text()[range.clone()];
        let display_start = self.display.len();
        self.map.push(range.start, display_start, range.len());
        self.display.push_str(text);
        if kind != SpanKind::Plain {
            self.spans.push(StyleSpan {
                range: display_start..self.display.len(),
                kind,
            });
        }
    }

    /// Emits display-only text with no source image (list markers). Display
    /// offsets inside it clamp back to the following mapped segment.
    fn push_display_only(&mut self, text: &str) {
        self.display.push_str(text);
    }

    fn emit_markup(&mut self, markup: ast::Markup<'a>, kind: SpanKind) {
        for expr in markup.exprs() {
            self.emit_inline(&expr, kind);
        }
    }

    /// Emits one inline expression. Styled constructs hide their markers by
    /// emitting only their bodies; everything unrecognized falls back to
    /// verbatim source, so the display never drops content.
    fn emit_inline(&mut self, expr: &ast::Expr<'a>, kind: SpanKind) {
        match expr {
            ast::Expr::Strong(strong) => self.emit_markup(strong.body(), SpanKind::Strong),
            ast::Expr::Emph(emph) => {
                let kind = if kind == SpanKind::Plain {
                    SpanKind::Emphasis
                } else {
                    kind
                };
                self.emit_markup(emph.body(), kind)
            }
            ast::Expr::Raw(raw) if !raw.block() => self.emit_raw_inner(raw),
            ast::Expr::Equation(equation) if !equation.block() => {
                self.push_verbatim(equation.body().to_untyped(), SpanKind::InlineMath)
            }
            _ => self.push_verbatim(expr.to_untyped(), kind),
        }
    }

    /// Emits a raw node's inner text (fences and language tag hidden). The
    /// newlines between a block raw's lines are source text too, so each
    /// gap is emitted from the source slice between line nodes.
    fn emit_raw_inner(&mut self, raw: &ast::Raw<'a>) {
        let mut previous_end: Option<usize> = None;
        for line in raw.lines() {
            let node = line.to_untyped();
            let range = self
                .document
                .node_range(node)
                .byte_range_in(self.document.source.text());
            if let Some(end) = previous_end {
                // The separator (newline + indentation) between raw lines.
                let separator = &self.document.source.text()[end..range.start];
                let display_start = self.display.len();
                self.map.push(end, display_start, separator.len());
                self.display.push_str(separator);
            }
            previous_end = Some(range.end);
            self.push_verbatim(node, SpanKind::InlineCode);
        }
    }

    fn finish(self, kind: BlockKind) -> Option<PreviewBlock> {
        let source_range = self.range?;
        Some(PreviewBlock::text(
            source_range,
            kind,
            self.display,
            self.spans,
            self.map,
        ))
    }
}
