use super::OffsetMap;
use crate::core::position::{Position, Range};

/// Models `*bold*` at source 10..16: the markers at 10 and 15 are hidden,
/// "bold" (source 11..15) shows at display 0..4.
fn strong_map() -> OffsetMap {
    let mut map = OffsetMap::default();
    map.push(11, 0, 4);
    map
}

#[test]
fn mapped_positions_round_trip() {
    let map = strong_map();
    assert_eq!(map.source_to_display(11), 0);
    assert_eq!(map.source_to_display(13), 2);
    assert_eq!(map.source_to_display(15), 4);
    assert_eq!(map.display_to_source(2), 13);
    assert_eq!(map.display_to_source(4), 15);
}

#[test]
fn hidden_marker_positions_clamp_to_neighbors() {
    let map = strong_map();
    // On the opening `*`: clamps forward to the visible text.
    assert_eq!(map.source_to_display(10), 0);
    // Past the closing `*`: clamps to the end of the visible text.
    assert_eq!(map.source_to_display(16), 4);
}

#[test]
fn multi_segment_maps_skip_gaps() {
    // "a *b* c" → display "a b c": segments (0..2)->(0..2), (3..4)->(2..3),
    // (5..7)->(3..5).
    let mut map = OffsetMap::default();
    map.push(0, 0, 2);
    map.push(3, 2, 1);
    map.push(5, 3, 2);

    assert_eq!(map.source_to_display(1), 1);
    assert_eq!(map.source_to_display(3), 2);
    assert_eq!(map.source_to_display(6), 4);
    assert_eq!(map.display_to_source(2), 3);
    assert_eq!(map.display_to_source(4), 6);
    // Display offsets past the end clamp to the last source position.
    assert_eq!(map.display_to_source(99), 7);
}

#[test]
fn adjacent_pushes_coalesce() {
    let mut map = OffsetMap::default();
    map.push(0, 0, 2);
    map.push(2, 2, 3);
    assert_eq!(map.source_to_display(4), 4);
    assert_eq!(map.display_to_source(5), 5);
}

#[test]
fn identity_map_is_verbatim() {
    let map = OffsetMap::identity(&Range::new(Position(10), Position(20)));
    assert_eq!(map.source_to_display(15), 5);
    assert_eq!(map.display_to_source(5), 15);
}

#[test]
fn empty_map_clamps_everything_to_zero() {
    let map = OffsetMap::default();
    assert_eq!(map.source_to_display(5), 0);
    assert_eq!(map.display_to_source(5), 0);
}
