pub use keyboard::{Event, KeyStatus, ScanCode};

pub enum Decoder {
    ReadNothing,
    ReadE0,
    ReadF0,
    ReadE012,
    ReadE012E0,
    ReadE0F0,
    ReadE0F07C,
    ReadE0F07CE0,
    ReadE0F07CE0F0,

    // This is only for the pause key
    ReadE1,
    ReadE114,
    ReadE1F0,
    ReadE1F014,
    ReadE1F014F0,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UnknownSequence;

pub type DecoderResult = Result<Option<Event>, UnknownSequence>;

impl Decoder {
    pub fn pressed(&mut self, scan_code: ScanCode) -> DecoderResult {
        *self = Self::ReadNothing;
        Ok(Some(Event {
            scan_code,
            key_status: KeyStatus::Pressed,
        }))
    }

    pub fn released(&mut self, scan_code: ScanCode) -> DecoderResult {
        *self = Self::ReadNothing;
        Ok(Some(Event {
            scan_code,
            key_status: KeyStatus::Released,
        }))
    }

    pub fn decode_pressed(&mut self, byte: u8, decode: decoder::Fn) -> DecoderResult {
        match decode(byte) {
            None => self.unknown(),
            Some(scan_code) => self.pressed(scan_code),
        }
    }

    pub fn decode_released(&mut self, byte: u8, decode: decoder::Fn) -> DecoderResult {
        match decode(byte) {
            None => self.unknown(),
            Some(scan_code) => self.released(scan_code),
        }
    }

    pub fn incomplete(&mut self, new_state: Self) -> DecoderResult {
        *self = new_state;
        Ok(None)
    }

    pub fn unknown(&mut self) -> DecoderResult {
        *self = Self::ReadNothing;
        Err(UnknownSequence)
    }

    pub fn transition(
        &mut self,
        received_byte: u8,
        required_byte: u8,
        next_state: Self,
    ) -> DecoderResult {
        match received_byte == required_byte {
            true => self.incomplete(next_state),
            false => self.unknown(),
        }
    }

    pub fn feed(&mut self, byte: u8) -> DecoderResult {
        use Decoder::*;
        use ScanCode::*;
        match self {
            ReadNothing => match byte {
                0xE0 => self.incomplete(ReadE0),
                0xE1 => self.incomplete(ReadE1),
                0xF0 => self.incomplete(ReadF0),
                _ => self.decode_pressed(byte, decoder::basic),
            },
            ReadE0 => match byte {
                0x12 => self.incomplete(ReadE012),
                0xF0 => self.incomplete(ReadE0F0),
                _ => self.decode_pressed(byte, decoder::extra),
            },
            ReadE012 => self.transition(byte, 0xE0, ReadE012E0),
            ReadE012E0 => match byte {
                0x7C => self.pressed(PrintScreen),
                _ => self.unknown(),
            },
            ReadE0F0 => match byte {
                0x7C => self.incomplete(ReadE0F07C),
                0x7D => self.released(PageUp),
                _ => self.decode_released(byte, decoder::extra),
            },
            ReadE0F07C => self.transition(byte, 0xE0, ReadE0F07CE0),
            ReadE0F07CE0 => self.transition(byte, 0xF0, ReadE0F07CE0F0),
            ReadE0F07CE0F0 => match byte {
                0x12 => self.released(PrintScreen),
                _ => self.unknown(),
            },
            ReadE1 => match byte {
                0x14 => self.incomplete(ReadE114),
                0xF0 => self.incomplete(ReadE1F0),
                _ => self.unknown(),
            },
            ReadE114 => match byte {
                0x77 => self.pressed(Pause),
                _ => self.unknown(),
            },
            ReadE1F0 => self.transition(byte, 0x14, ReadE1F014),
            ReadE1F014 => self.transition(byte, 0xF0, ReadE1F014F0),
            ReadE1F014F0 => match byte {
                0x77 => self.released(Pause),
                _ => self.unknown(),
            },
            ReadF0 => self.decode_released(byte, decoder::basic),
        }
    }
}

mod decoder {
    use keyboard::ScanCode;

    pub type Fn = fn(u8) -> Option<ScanCode>;

    pub fn basic(byte: u8) -> Option<ScanCode> {
        use ScanCode::*;
        Some(match byte {
            0x01 => F9,
            0x03 => F5,
            0x04 => F3,
            0x05 => F1,
            0x06 => F2,
            0x07 => F12,
            0x09 => F10,
            0x0A => F8,
            0x0B => F6,
            0x0C => F4,
            0x0D => Tab,
            0x0E => BackTick,
            0x11 => LeftAlt,
            0x12 => LeftShift,
            0x14 => LeftControl,
            0x15 => Q,
            0x16 => One,
            0x1A => Z,
            0x1B => S,
            0x1C => A,
            0x1D => W,
            0x1E => Two,
            0x21 => C,
            0x22 => X,
            0x23 => D,
            0x24 => E,
            0x25 => Four,
            0x26 => Three,
            0x29 => Space,
            0x2A => V,
            0x2B => F,
            0x2C => T,
            0x2D => R,
            0x2E => Five,
            0x31 => N,
            0x32 => B,
            0x33 => H,
            0x34 => G,
            0x35 => Y,
            0x36 => Six,
            0x3A => M,
            0x3B => J,
            0x3C => U,
            0x3D => Seven,
            0x3E => Eight,
            0x41 => Comma,
            0x42 => K,
            0x43 => I,
            0x44 => O,
            0x45 => Zero,
            0x46 => Nine,
            0x49 => Dot,
            0x4A => Slash,
            0x4B => L,
            0x4C => SemiColon,
            0x4D => P,
            0x4E => Minus,
            0x52 => SingleQuote,
            0x54 => LeftBracket,
            0x55 => Equal,
            0x58 => CapsLock,
            0x59 => RightShift,
            0x5A => Enter,
            0x5B => RightBracket,
            0x5D => BackSlash,
            0x66 => Backspace,
            0x69 => KeypadOne,
            0x6B => KeypadFour,
            0x6C => KeypadSeven,
            0x70 => KeypadZero,
            0x71 => KeypadDot,
            0x72 => KeypadTwo,
            0x73 => KeypadFive,
            0x74 => KeypadSix,
            0x75 => KeypadEight,
            0x76 => Escape,
            0x77 => NumberLock,
            0x78 => F11,
            0x79 => KeypadPlus,
            0x7A => KeypadThree,
            0x7B => KeypadMinus,
            0x7C => KeypadStar,
            0x7D => KeypadNine,
            0x7E => ScrollLock,
            0x83 => F7,
            _ => return None,
        })
    }

    pub fn extra(byte: u8) -> Option<ScanCode> {
        use ScanCode::*;
        Some(match byte {
            0x10 => WebSearch,
            0x11 => RightAlt,
            0x14 => RightControl,
            0x15 => PreviousTrack,
            0x18 => WebFavorites,
            0x1F => LeftGUI,
            0x20 => WebRefresh,
            0x21 => VolumeDown,
            0x23 => Mute,
            0x27 => RightGUI,
            0x28 => WebStop,
            0x2B => Calculator,
            0x2F => Apps,
            0x30 => WebForward,
            0x32 => VolumeUp,
            0x34 => PlayPause,
            0x37 => Power,
            0x38 => WebBack,
            0x3A => WebHome,
            0x3B => Stop,
            0x3F => Sleep,
            0x40 => MyComputer,
            0x48 => Email,
            0x4A => KeypadSlash,
            0x4D => NextTrack,
            0x50 => MediaSelect,
            0x5A => KeypadEnter,
            0x5E => Wake,
            0x69 => End,
            0x6B => CursorLeft,
            0x6C => Home,
            0x70 => Insert,
            0x71 => Delete,
            0x72 => CursorDown,
            0x74 => CursorRight,
            0x75 => CursorUp,
            0x7A => PageDown,
            0x7D => PageUp,
            _ => return None,
        })
    }
}
