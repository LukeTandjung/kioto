use gpui::{HighlightStyle, font, red, white};

use super::{Highlight, runs_for_line};

fn color_highlight(range: std::ops::Range<usize>) -> Highlight {
    Highlight {
        range,
        style: HighlightStyle {
            color: Some(red()),
            ..Default::default()
        },
    }
}

#[test]
fn no_highlights_yield_one_base_run() {
    let runs = runs_for_line(&(10..15), font("Test"), white(), &[]);
    assert_eq!(runs.len(), 1);
    assert_eq!(runs[0].len, 5);
    assert_eq!(runs[0].color, white());
}

#[test]
fn highlight_splits_line_into_segments() {
    let runs = runs_for_line(&(10..20), font("Test"), white(), &[color_highlight(12..15)]);
    let lens: Vec<usize> = runs.iter().map(|run| run.len).collect();
    assert_eq!(lens, vec![2, 3, 5]);
    assert_eq!(runs[1].color, red());
    assert_eq!(runs[0].color, white());
}

#[test]
fn highlights_outside_the_line_are_ignored() {
    let runs = runs_for_line(&(10..20), font("Test"), white(), &[color_highlight(0..5)]);
    assert_eq!(runs.len(), 1);
    assert_eq!(runs[0].color, white());
}

#[test]
fn later_highlights_override_earlier_ones() {
    let over = Highlight {
        range: 12..14,
        style: HighlightStyle {
            color: Some(white()),
            ..Default::default()
        },
    };
    let runs = runs_for_line(
        &(10..20),
        font("Test"),
        white(),
        &[color_highlight(10..16), over],
    );
    // Segments: 10..12 red, 12..14 white (override), 14..16 red, 16..20 base.
    assert_eq!(runs.iter().map(|run| run.len).sum::<usize>(), 10);
    assert_eq!(runs[0].color, red());
    assert_eq!(runs[1].color, white());
    assert_eq!(runs[2].color, red());
}

#[test]
fn empty_line_keeps_a_zero_length_base_run() {
    let runs = runs_for_line(&(10..10), font("Test"), white(), &[color_highlight(0..20)]);
    assert_eq!(runs.len(), 1);
    assert_eq!(runs[0].len, 0);
}

#[test]
fn highlight_spanning_a_wrap_boundary_clips_to_the_row() {
    // The highlight covers 8..12 but this display row shows only 10..20.
    let runs = runs_for_line(&(10..20), font("Test"), white(), &[color_highlight(8..12)]);
    let lens: Vec<usize> = runs.iter().map(|run| run.len).collect();
    assert_eq!(lens, vec![2, 8]);
    assert_eq!(runs[0].color, red());
}
