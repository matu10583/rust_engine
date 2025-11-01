// Engine-level input abstractions. These are backend-agnostic enums that
// the rest of the engine should use. Platform-specific runners should map
// their backend events (winit, etc.) into these types.

/// Logical engine key. Fully enumerated so the engine public API does not
/// directly expose backend types.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EngineElementState {
    Pressed,
    Released,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EngineKey {
    // Letters
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,

    // Numbers (top row)
    Key0,
    Key1,
    Key2,
    Key3,
    Key4,
    Key5,
    Key6,
    Key7,
    Key8,
    Key9,

    // Function keys
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,

    // Navigation
    Escape,
    Tab,
    CapsLock,
    ShiftLeft,
    ShiftRight,
    ControlLeft,
    ControlRight,
    AltLeft,
    AltRight,
    Space,
    Enter,
    Backspace,
    Insert,
    Delete,
    Home,
    End,
    PageUp,
    PageDown,
    ArrowUp,
    ArrowDown,
    ArrowLeft,
    ArrowRight,

    // Numpad
    NumLock,
    Numpad0,
    Numpad1,
    Numpad2,
    Numpad3,
    Numpad4,
    Numpad5,
    Numpad6,
    Numpad7,
    Numpad8,
    Numpad9,
    NumpadAdd,
    NumpadSubtract,
    NumpadMultiply,
    NumpadDivide,
    NumpadDecimal,

    // Misc / media / punctuation
    PrintScreen,
    Pause,
    ScrollLock,
    Minus,
    Equals,
    LeftBracket,
    RightBracket,
    Backslash,
    Semicolon,
    Apostrophe,
    Grave,
    Comma,
    Period,
    Slash,

    // Raw fallback for keys not enumerated here or future additions.
    Raw(u32),
}

/// Mouse buttons at engine level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EngineMouseButton {
    Left,
    Right,
    Middle,
    Other(u16),
}

// Conversion implementations for the winit backend.
use winit::event::{ElementState, MouseButton, VirtualKeyCode};

impl From<VirtualKeyCode> for EngineKey {
    fn from(v: VirtualKeyCode) -> Self {
        // Conservative default mapping: map everything to Raw by default.
        // This avoids brittle exhaustive matching across winit versions.
        EngineKey::Raw(v as u32)
    }
}

impl From<MouseButton> for EngineMouseButton {
    fn from(m: MouseButton) -> Self {
        match m {
            MouseButton::Left => EngineMouseButton::Left,
            MouseButton::Right => EngineMouseButton::Right,
            MouseButton::Middle => EngineMouseButton::Middle,
            MouseButton::Other(n) => EngineMouseButton::Other(n),
        }
    }
}

impl From<ElementState> for EngineElementState {
    fn from(state: ElementState) -> Self {
        match state {
            ElementState::Pressed => EngineElementState::Pressed,
            ElementState::Released => EngineElementState::Released,
        }
    }
}
