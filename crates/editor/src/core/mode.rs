use crate::core::motion::Motion;

/// A single normalized input event, produced by the view layer from raw
/// keystrokes. Committed text insertion (including IME) also arrives as
/// `EditorAction::InsertText` directly from the platform input path.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Input {
    Char(char),
    Escape,
    Enter,
    Backspace,
    Delete,
    Left,
    Right,
    Up,
    Down,
    Home,
    End,
}

/// What a mode wants done, as pure data. Actions whose interpretation needs
/// side effects (paste, yank, save) are interpreted in `app`, which holds
/// the ports; everything else is applied against the buffer and cursors.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EditorAction {
    None,
    InsertText(String),
    DeleteBackward,
    DeleteForward,
    Move(Motion),
    Extend(Motion),
    EnterInsert { after_cursor: bool },
    EnterNormal,
    EnterVisual,
    SelectLine,
    DeleteSelection,
    // Interpreted via ports / core history from milestone 5:
    Yank,
    Paste,
    Undo,
    StartSearch,
    NextMatch,
    PreviousMatch,
}

/// The modal state machine contract. `&mut self` because multi-key
/// sequences (`g g`, and later counts and operators) carry transient state
/// between keystrokes.
pub trait Mode {
    fn handle_input(&mut self, input: Input) -> EditorAction;
}

#[derive(Debug, Default)]
pub struct InsertMode;

#[derive(Debug, Default)]
pub struct NormalMode {
    pending_g: bool,
}

#[derive(Debug, Default)]
pub struct VisualMode;

impl Mode for InsertMode {
    fn handle_input(&mut self, input: Input) -> EditorAction {
        match input {
            Input::Escape => EditorAction::EnterNormal,
            Input::Enter => EditorAction::InsertText("\n".into()),
            Input::Backspace => EditorAction::DeleteBackward,
            Input::Delete => EditorAction::DeleteForward,
            Input::Char(character) => EditorAction::InsertText(character.to_string()),
            _ => motion_for_input(input)
                .map(EditorAction::Move)
                .unwrap_or(EditorAction::None),
        }
    }
}

impl Mode for NormalMode {
    fn handle_input(&mut self, input: Input) -> EditorAction {
        if self.pending_g {
            self.pending_g = false;
            return match input {
                Input::Char('g') => EditorAction::Move(Motion::DocumentStart),
                _ => EditorAction::None,
            };
        }

        match input {
            Input::Char('g') => {
                self.pending_g = true;
                EditorAction::None
            }
            Input::Char('G') => EditorAction::Move(Motion::DocumentEnd),
            Input::Char('i') => EditorAction::EnterInsert {
                after_cursor: false,
            },
            Input::Char('a') => EditorAction::EnterInsert { after_cursor: true },
            Input::Char('v') => EditorAction::EnterVisual,
            Input::Char('x') => EditorAction::SelectLine,
            Input::Char('d') => EditorAction::DeleteSelection,
            Input::Char('y') => EditorAction::Yank,
            Input::Char('p') => EditorAction::Paste,
            Input::Char('u') => EditorAction::Undo,
            Input::Char('/') => EditorAction::StartSearch,
            Input::Char('n') => EditorAction::NextMatch,
            Input::Char('N') => EditorAction::PreviousMatch,
            _ => motion_for_input(input)
                .map(EditorAction::Move)
                .unwrap_or(EditorAction::None),
        }
    }
}

impl Mode for VisualMode {
    fn handle_input(&mut self, input: Input) -> EditorAction {
        match input {
            Input::Escape | Input::Char('v') => EditorAction::EnterNormal,
            Input::Char('d') => EditorAction::DeleteSelection,
            Input::Char('y') => EditorAction::Yank,
            Input::Char('x') => EditorAction::SelectLine,
            Input::Char('G') => EditorAction::Extend(Motion::DocumentEnd),
            _ => motion_for_input(input)
                .map(EditorAction::Extend)
                .unwrap_or(EditorAction::None),
        }
    }
}

/// The shared key → motion table (`h j k l`, `w b e`, `0 $`, arrows,
/// home/end). Modes decide whether a motion moves or extends.
fn motion_for_input(input: Input) -> Option<Motion> {
    match input {
        Input::Char('h') | Input::Left => Some(Motion::Left),
        Input::Char('j') | Input::Down => Some(Motion::Down),
        Input::Char('k') | Input::Up => Some(Motion::Up),
        Input::Char('l') | Input::Right => Some(Motion::Right),
        Input::Char('w') => Some(Motion::WordForward),
        Input::Char('b') => Some(Motion::WordBack),
        Input::Char('e') => Some(Motion::WordEnd),
        Input::Char('0') | Input::Home => Some(Motion::LineStart),
        Input::Char('$') | Input::End => Some(Motion::LineEnd),
        _ => None,
    }
}

/// The active editor mode: an enum wrapper so `Editor` owns one value while
/// each concrete mode keeps its own transient state.
#[derive(Debug)]
pub enum EditorMode {
    Insert(InsertMode),
    Normal(NormalMode),
    Visual(VisualMode),
}

impl EditorMode {
    pub fn insert() -> Self {
        Self::Insert(InsertMode)
    }

    pub fn normal() -> Self {
        Self::Normal(NormalMode::default())
    }

    pub fn visual() -> Self {
        Self::Visual(VisualMode)
    }

    pub fn handle_input(&mut self, input: Input) -> EditorAction {
        match self {
            Self::Insert(mode) => mode.handle_input(input),
            Self::Normal(mode) => mode.handle_input(input),
            Self::Visual(mode) => mode.handle_input(input),
        }
    }

    pub fn is_insert(&self) -> bool {
        matches!(self, Self::Insert(_))
    }

    pub fn is_visual(&self) -> bool {
        matches!(self, Self::Visual(_))
    }

    pub fn label(&self) -> &'static str {
        match self {
            Self::Insert(_) => "INSERT",
            Self::Normal(_) => "NORMAL",
            Self::Visual(_) => "VISUAL",
        }
    }
}

#[cfg(test)]
#[path = "mode.test.rs"]
mod tests;
