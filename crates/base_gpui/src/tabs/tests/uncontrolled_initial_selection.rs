use gpui::TestAppContext;

use super::support::{OVERVIEW, TabsTestConfig, read_observations, open_tabs};

#[gpui::test]
fn uncontrolled_tabs_select_first_enabled_tab(cx: &mut TestAppContext) {
    let window = open_tabs(cx, TabsTestConfig::default());
    let observations = read_observations(cx, window);

    assert_eq!(observations.active_value(), Some(OVERVIEW));
    assert_eq!(observations.highlighted_value(), Some(OVERVIEW));
    assert!(observations.value_changes.is_empty());
}
