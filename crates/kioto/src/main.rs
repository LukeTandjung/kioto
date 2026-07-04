use editor::{EditorConfig, create_editor};
use gpui::*;
use gpui_component::*;
use gpui_platform::application;

const SAMPLE: &str = r#"= Metric Spaces

A *metric space* is a set $M$ together with a _distance_
function $d: M times M -> RR$ satisfying, for all $x, y, z in M$:

+ $d(x, y) >= 0$ — non-negativity
+ $d(x, y) = 0 <=> x = y$ — identity of indiscernibles
+ $d(x, y) = d(y, x)$ — symmetry
+ $d(x, z) <= d(x, y) + d(y, z)$ — triangle inequality

The triangle inequality generalizes to finite chains of points:

$ d(x_0, x_n) <= sum_(i=1)^n d(x_(i-1), x_i) $

== Convergence

A sequence $(x_n)$ *converges* to $x$ if for every $epsilon > 0$
there exists $N$ such that $d(x_n, x) < epsilon$ whenever $n >= N$.

== Completeness

A metric space is _complete_ when every Cauchy sequence
converges to a point within the space. $RR$ is complete; $QQ$ is not.
"#;

fn main() {
    application().run(move |cx: &mut App| {
        // This must be called before using any GPUI Component features.
        gpui_component::init(cx);

        cx.spawn(async move |cx| {
            cx.open_window(WindowOptions::default(), |window, cx| {
                // The sample seeds the buffer on first run; once saved
                // (ctrl-s), later runs load the file instead.
                let location = std::path::PathBuf::from("metric-spaces.typ");
                let text = if location.exists() {
                    String::new()
                } else {
                    SAMPLE.into()
                };
                let view = create_editor(
                    EditorConfig {
                        text,
                        title: "metric-spaces.typ".into(),
                        location: Some(location),
                        ..Default::default()
                    },
                    window,
                    cx,
                );

                // This first level on the window, should be a Root.
                cx.new(|cx| Root::new(view, window, cx))
            })
            .expect("Failed to open window");
        })
        .detach();
    });
}
