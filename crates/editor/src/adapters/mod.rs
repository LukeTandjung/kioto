// Side-effecting implementations of ports (filesystem store, GPUI
// clipboard), plus an in-memory clipboard for tests and headless use.
pub mod filesystem_store;
pub mod gpui_clipboard;
pub mod memory_clipboard;
