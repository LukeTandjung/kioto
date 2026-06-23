use gpui::{App, KeyBinding, actions};

pub const EDITOR_INSERT_CONTEXT: &str = "KiotoEditorInsert";
pub const EDITOR_NORMAL_CONTEXT: &str = "KiotoEditorNormal";
pub const EDITOR_SELECT_CONTEXT: &str = "KiotoEditorSelect";

actions!(
    editor,
    [
        Backspace,
        Delete,
        Enter,
        MoveLeft,
        MoveRight,
        MoveUp,
        MoveDown,
        MoveLineStart,
        MoveLineEnd,
        SelectAll,
        Copy,
        Cut,
        Paste,
        NormalMode,
        InsertMode,
        AppendMode,
        SelectMode,
        SelectLine,
    ]
);

pub fn init(cx: &mut App) {
    cx.bind_keys([
        KeyBinding::new("escape", NormalMode, Some(EDITOR_INSERT_CONTEXT)),
        KeyBinding::new("backspace", Backspace, Some(EDITOR_INSERT_CONTEXT)),
        KeyBinding::new("delete", Delete, Some(EDITOR_INSERT_CONTEXT)),
        KeyBinding::new("enter", Enter, Some(EDITOR_INSERT_CONTEXT)),
        KeyBinding::new("left", MoveLeft, Some(EDITOR_INSERT_CONTEXT)),
        KeyBinding::new("right", MoveRight, Some(EDITOR_INSERT_CONTEXT)),
        KeyBinding::new("up", MoveUp, Some(EDITOR_INSERT_CONTEXT)),
        KeyBinding::new("down", MoveDown, Some(EDITOR_INSERT_CONTEXT)),
        KeyBinding::new("home", MoveLineStart, Some(EDITOR_INSERT_CONTEXT)),
        KeyBinding::new("end", MoveLineEnd, Some(EDITOR_INSERT_CONTEXT)),
        #[cfg(target_os = "macos")]
        KeyBinding::new("cmd-a", SelectAll, Some(EDITOR_INSERT_CONTEXT)),
        #[cfg(not(target_os = "macos"))]
        KeyBinding::new("ctrl-a", SelectAll, Some(EDITOR_INSERT_CONTEXT)),
        #[cfg(target_os = "macos")]
        KeyBinding::new("cmd-c", Copy, Some(EDITOR_INSERT_CONTEXT)),
        #[cfg(not(target_os = "macos"))]
        KeyBinding::new("ctrl-c", Copy, Some(EDITOR_INSERT_CONTEXT)),
        #[cfg(target_os = "macos")]
        KeyBinding::new("cmd-x", Cut, Some(EDITOR_INSERT_CONTEXT)),
        #[cfg(not(target_os = "macos"))]
        KeyBinding::new("ctrl-x", Cut, Some(EDITOR_INSERT_CONTEXT)),
        #[cfg(target_os = "macos")]
        KeyBinding::new("cmd-v", Paste, Some(EDITOR_INSERT_CONTEXT)),
        #[cfg(not(target_os = "macos"))]
        KeyBinding::new("ctrl-v", Paste, Some(EDITOR_INSERT_CONTEXT)),
        KeyBinding::new("i", InsertMode, Some(EDITOR_NORMAL_CONTEXT)),
        KeyBinding::new("a", AppendMode, Some(EDITOR_NORMAL_CONTEXT)),
        KeyBinding::new("v", SelectMode, Some(EDITOR_NORMAL_CONTEXT)),
        KeyBinding::new("x", SelectLine, Some(EDITOR_NORMAL_CONTEXT)),
        KeyBinding::new("y", Copy, Some(EDITOR_NORMAL_CONTEXT)),
        KeyBinding::new("d", Cut, Some(EDITOR_NORMAL_CONTEXT)),
        KeyBinding::new("p", Paste, Some(EDITOR_NORMAL_CONTEXT)),
        KeyBinding::new("h", MoveLeft, Some(EDITOR_NORMAL_CONTEXT)),
        KeyBinding::new("j", MoveDown, Some(EDITOR_NORMAL_CONTEXT)),
        KeyBinding::new("k", MoveUp, Some(EDITOR_NORMAL_CONTEXT)),
        KeyBinding::new("l", MoveRight, Some(EDITOR_NORMAL_CONTEXT)),
        KeyBinding::new("left", MoveLeft, Some(EDITOR_NORMAL_CONTEXT)),
        KeyBinding::new("right", MoveRight, Some(EDITOR_NORMAL_CONTEXT)),
        KeyBinding::new("up", MoveUp, Some(EDITOR_NORMAL_CONTEXT)),
        KeyBinding::new("down", MoveDown, Some(EDITOR_NORMAL_CONTEXT)),
        KeyBinding::new("home", MoveLineStart, Some(EDITOR_NORMAL_CONTEXT)),
        KeyBinding::new("end", MoveLineEnd, Some(EDITOR_NORMAL_CONTEXT)),
        #[cfg(target_os = "macos")]
        KeyBinding::new("cmd-a", SelectAll, Some(EDITOR_NORMAL_CONTEXT)),
        #[cfg(not(target_os = "macos"))]
        KeyBinding::new("ctrl-a", SelectAll, Some(EDITOR_NORMAL_CONTEXT)),
        KeyBinding::new("escape", NormalMode, Some(EDITOR_SELECT_CONTEXT)),
        KeyBinding::new("v", NormalMode, Some(EDITOR_SELECT_CONTEXT)),
        KeyBinding::new("i", InsertMode, Some(EDITOR_SELECT_CONTEXT)),
        KeyBinding::new("a", AppendMode, Some(EDITOR_SELECT_CONTEXT)),
        KeyBinding::new("x", SelectLine, Some(EDITOR_SELECT_CONTEXT)),
        KeyBinding::new("y", Copy, Some(EDITOR_SELECT_CONTEXT)),
        KeyBinding::new("d", Cut, Some(EDITOR_SELECT_CONTEXT)),
        KeyBinding::new("h", MoveLeft, Some(EDITOR_SELECT_CONTEXT)),
        KeyBinding::new("j", MoveDown, Some(EDITOR_SELECT_CONTEXT)),
        KeyBinding::new("k", MoveUp, Some(EDITOR_SELECT_CONTEXT)),
        KeyBinding::new("l", MoveRight, Some(EDITOR_SELECT_CONTEXT)),
        KeyBinding::new("left", MoveLeft, Some(EDITOR_SELECT_CONTEXT)),
        KeyBinding::new("right", MoveRight, Some(EDITOR_SELECT_CONTEXT)),
        KeyBinding::new("up", MoveUp, Some(EDITOR_SELECT_CONTEXT)),
        KeyBinding::new("down", MoveDown, Some(EDITOR_SELECT_CONTEXT)),
        KeyBinding::new("home", MoveLineStart, Some(EDITOR_SELECT_CONTEXT)),
        KeyBinding::new("end", MoveLineEnd, Some(EDITOR_SELECT_CONTEXT)),
        #[cfg(target_os = "macos")]
        KeyBinding::new("cmd-a", SelectAll, Some(EDITOR_SELECT_CONTEXT)),
        #[cfg(not(target_os = "macos"))]
        KeyBinding::new("ctrl-a", SelectAll, Some(EDITOR_SELECT_CONTEXT)),
    ]);
}
