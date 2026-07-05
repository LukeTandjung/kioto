use std::sync::Arc;

use crate::app::documents::fragments::FragmentCompiler;

#[test]
fn compiles_math_to_a_bitmap() {
    let compiler = FragmentCompiler::new();
    let fragment = compiler
        .render_math("integral_0^1 x^2 dif x = 1/3")
        .expect("valid math should compile");

    assert!(fragment.width > 0);
    assert!(fragment.height > 0);
    assert_eq!(
        fragment.bgra.len(),
        fragment.width as usize * fragment.height as usize * 4
    );
    assert_eq!(fragment.logical_width, fragment.width as f32 / 2.0);
    assert_eq!(fragment.logical_height, fragment.height as f32 / 2.0);
}

#[test]
fn caches_by_source_text() {
    let compiler = FragmentCompiler::new();
    let first = compiler.render_math("x + y").expect("should compile");
    let second = compiler.render_math("x + y").expect("should compile");
    assert!(Arc::ptr_eq(&first, &second));

    let different = compiler.render_math("x - y").expect("should compile");
    assert!(!Arc::ptr_eq(&first, &different));
}

#[test]
fn inline_and_display_styles_are_cached_separately() {
    let compiler = FragmentCompiler::new();
    let display = compiler
        .render_math("sum_(i=1)^n i")
        .expect("should compile");
    let inline = compiler
        .render_inline_math("sum_(i=1)^n i")
        .expect("should compile");
    assert!(!Arc::ptr_eq(&display, &inline));
    // Inline style shrinks the limits, so the fragment is shorter.
    assert!(inline.height < display.height);
}

#[test]
fn broken_markup_compiles_to_none() {
    let compiler = FragmentCompiler::new();
    // An undefined function is a compile error, not a panic.
    assert!(compiler.render_math("nosuchfunction()").is_none());
    // And the failure is cached like any other result.
    assert!(compiler.render_math("nosuchfunction()").is_none());
}
