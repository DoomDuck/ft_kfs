
use super::{Char, Color, TextBuffer};

pub struct Screen {
    chars: [[Char; Self::WIDTH]; Self::HEIGHT],
}

impl Screen {
    pub const WIDTH: usize = TextBuffer::WIDTH;
    pub const HEIGHT: usize = TextBuffer::HEIGHT;
}
