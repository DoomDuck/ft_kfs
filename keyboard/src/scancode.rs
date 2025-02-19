#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ScanCode {
    // Number
    Zero = 0,
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,

    // Letters
    A = 10,
    B = 11,
    C = 12,
    D = 13,
    E = 14,
    F = 15,
    G = 16,
    H = 17,
    I = 18,
    J = 19,
    K = 20,
    L = 21,
    M = 22,
    N = 23,
    O = 24,
    P = 25,
    Q = 26,
    R = 27,
    S = 28,
    T = 29,
    U = 30,
    V = 31,
    W = 32,
    X = 33,
    Y = 34,
    Z = 35,

    // TODO(Dorian):  Fix the hole in the table

    // Cursor (arrow) keys
    CursorLeft = 37,
    CursorRight = 38,
    CursorUp = 39,
    CursorDown = 40,

    // Function
    F1 = 41,
    F2 = 42,
    F3 = 43,
    F4 = 44,
    F5 = 45,
    F6 = 46,
    F7 = 47,
    F8 = 48,
    F9 = 49,
    F10 = 50,
    F11 = 51,
    F12 = 52,

    // Action
    Escape = 53,
    Tab = 54,
    Enter = 55,
    Space = 56,
    Backspace = 57,
    Delete = 58,

    // Symbols
    Minus = 59,
    Equal = 60,
    LeftBracket = 61,
    RightBracket = 62,
    BackTick = 63,
    SingleQuote = 64,
    Slash = 65,
    BackSlash = 66,
    Comma = 67,
    Dot = 68,
    SemiColon = 69,
    Star = 70,

    // Modifiers
    LeftShift = 71,
    RightShift = 72,
    LeftControl = 73,
    RightControl = 74,
    LeftAlt = 75,
    RightAlt = 76,

    // Locks
    CapsLock = 77,
    NumberLock = 78,
    ScrollLock = 79,

    // Keypad
    KeypadZero = 80,
    KeypadOne = 81,
    KeypadTwo = 82,
    KeypadThree = 83,
    KeypadFour = 84,
    KeypadFive = 85,
    KeypadSix = 86,
    KeypadSeven = 87,
    KeypadEight = 88,
    KeypadNine = 89,
    KeypadMinus = 90,
    KeypadPlus = 91,
    KeypadDot = 92,
    KeypadSlash = 93,
    KeypadStar = 94,
    KeypadEnter = 95,

    // Insert
    Insert = 96,

    // Audio
    PreviousTrack = 97,
    NextTrack = 98,
    Mute = 99,
    Play = 100,
    Pause = 101,
    PlayPause = 102,
    Stop = 103,

    // Volume
    VolumeDown = 104,
    VolumeUp = 105,

    // Page
    PageDown = 106,
    PageUp = 107,
    End = 108,

    // GUI (:shrug:)
    LeftGUI = 109,
    RightGUI = 110,

    // Power management
    Power = 111,
    Sleep = 112,
    Wake = 113,

    // Web
    WebSearch = 114,
    WebForward = 115,
    WebBack = 116,
    WebHome = 117,
    WebFavorites = 118,
    WebRefresh = 119,
    WebStop = 120,

    // Desktop
    Apps = 121,
    Home = 122,
    Email = 123,
    MyComputer = 124,
    Calculator = 125,
    MediaSelect = 126,
    PrintScreen = 127,
}

impl ScanCode {
    pub const COUNT: usize = 128;

    pub const fn is_basic_number(self) -> bool {
        Self::Zero as u8 <= self as u8 && self as u8 <= Self::Nine as u8
    }

    pub const fn is_keypad_number(self) -> bool {
        Self::KeypadZero as u8 <= self as u8 && self as u8 <= Self::KeypadNine as u8
    }

    pub const fn is_number(self) -> bool {
        self.is_basic_number() || self.is_keypad_number()
    }

    pub const fn is_letter(self) -> bool {
        Self::A as u8 <= self as u8 && self as u8 <= Self::Z as u8
    }

    pub const fn is_basic_symbol(self) -> bool {
        Self::Minus as u8 <= self as u8 && self as u8 <= Self::Star as u8
    }

    pub const fn is_keypad_symbol(self) -> bool {
        Self::KeypadMinus as u8 <= self as u8 && self as u8 <= Self::KeypadStar as u8
    }

    pub const fn is_symbol(self) -> bool {
        self.is_basic_symbol() || self.is_keypad_symbol()
    }

    pub const fn try_into_ascii(self) -> Option<u8> {
        Some(if self.is_basic_number() {
            b'0' + (self as u8 - Self::Zero as u8)
        } else if self.is_keypad_number() {
            b'0' + (self as u8 - Self::KeypadZero as u8)
        } else if self.is_letter() {
            b'A' + (self as u8 - Self::A as u8)
        } else {
            match self {
                Self::Space => b' ',
                Self::Minus | Self::KeypadMinus => b'-',
                Self::Equal => b'=',
                Self::LeftBracket => b'[',
                Self::RightBracket => b']',
                Self::BackTick => b'`',
                Self::SingleQuote => b'\'',
                Self::Slash | Self::KeypadSlash => b'/',
                Self::BackSlash => b'\\',
                Self::Comma => b',',
                Self::Dot | Self::KeypadDot => b'.',
                Self::SemiColon => b';',
                Self::Star | Self::KeypadStar => b'*',
                Self::KeypadPlus => b'+',
                _ => return None,
            }
        })
    }
}
