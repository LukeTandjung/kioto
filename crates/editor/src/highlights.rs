use std::ops::Range;

use gpui::{Font, HighlightStyle, Hsla, TextRun};

/// One styled region from an independent highlight source (IME marked text,
/// block cursor, later syntax/diagnostics/search), in buffer byte offsets.
pub struct Highlight {
    pub range: Range<usize>,
    pub style: HighlightStyle,
}

/// Merges highlight sources into the text runs for one display row. Sources
/// are applied in order, so later highlights override earlier ones where
/// they overlap; unset style fields fall through to the base style.
pub fn runs_for_line(
    line_range: &Range<usize>,
    font: Font,
    color: Hsla,
    highlights: &[Highlight],
) -> Vec<TextRun> {
    let base = TextRun {
        len: line_range.len(),
        font,
        color,
        background_color: None,
        underline: None,
        strikethrough: None,
    };

    let clip = |range: &Range<usize>| {
        let start = range.start.clamp(line_range.start, line_range.end) - line_range.start;
        let end = range.end.clamp(line_range.start, line_range.end) - line_range.start;
        start..end
    };

    let mut cuts = vec![0, line_range.len()];
    for highlight in highlights {
        let local = clip(&highlight.range);
        if !local.is_empty() {
            cuts.push(local.start);
            cuts.push(local.end);
        }
    }
    cuts.sort_unstable();
    cuts.dedup();

    let mut runs = Vec::with_capacity(cuts.len().saturating_sub(1));
    for segment in cuts.windows(2) {
        let (start, end) = (segment[0], segment[1]);
        if start == end {
            continue;
        }

        let mut run = TextRun {
            len: end - start,
            ..base.clone()
        };
        for highlight in highlights {
            let local = clip(&highlight.range);
            if local.start <= start && end <= local.end {
                if let Some(color) = highlight.style.color {
                    run.color = color;
                }
                if let Some(background) = highlight.style.background_color {
                    run.background_color = Some(background);
                }
                if let Some(underline) = highlight.style.underline {
                    run.underline = Some(underline);
                }
                if let Some(strikethrough) = highlight.style.strikethrough {
                    run.strikethrough = Some(strikethrough);
                }
            }
        }
        runs.push(run);
    }

    if runs.is_empty() {
        return vec![base];
    }
    runs
}

#[cfg(test)]
#[path = "highlights.test.rs"]
mod tests;
