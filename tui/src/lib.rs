use vga::Char;

pub struct Screen {
    chars: [[Char; Self::WIDTH]; Self::HEIGHT],
}

impl Screen {
    pub const WIDTH: usize = vga::TextBuffer::WIDTH;
    pub const HEIGHT: usize = vga::TextBuffer::HEIGHT;
}
