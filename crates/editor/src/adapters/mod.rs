// Side-effecting implementations of ports (filesystem store, GPUI
// clipboard), plus test-only adapter fixtures.
mod main;
#[cfg(test)]
mod test;

pub use main::*;
#[cfg(test)]
pub use test::memory_clipboard;
