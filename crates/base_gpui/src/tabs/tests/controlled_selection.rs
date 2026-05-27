use gpui::TestAppContext;

use super::support::{
    OVERVIEW, PROJECTS, TabsTestConfig, read_observations, open_tabs, simulate_keys, update_config,
};

#[gpui::test]
fn controlled_tabs_notify_without_mutating_internal_selection(cx: &mut TestAppContext) {
    let window = open_tabs(
        cx,
        TabsTestConfig {
            controlled_value: Some(Some(OVERVIEW)),
            ..TabsTestConfig::default()
        },
    );

    simulate_keys(cx, window, "right enter");

    let observations = read_observations(cx, window);
    assert_eq!(observations.value_changes, vec![Some(PROJECTS)]);
    assert_eq!(observations.active_value(), Some(OVERVIEW));

    update_config(cx, window, |config| {
        config.controlled_value = Some(Some(PROJECTS));
    });

    assert_eq!(read_observations(cx, window).active_value(), Some(PROJECTS));
}
