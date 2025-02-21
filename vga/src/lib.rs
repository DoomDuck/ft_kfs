#![no_std]

mod screen;

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Color(u8);

impl Default for Color {
    fn default() -> Self {
        Self::new(Self::WHITE, Self::BLACK)
    }
}

impl Color {
    pub const BLACK: u8 = 0x0;
    pub const BLUE: u8 = 0x1;
    pub const GREEN: u8 = 0x2;
    pub const CYAN: u8 = 0x3;
    pub const RED: u8 = 0x4;
    pub const MAGENTA: u8 = 0x5;
    pub const BROWN: u8 = 0x6;
    pub const LIGHT_GRAY: u8 = 0x7;
    pub const DARK_GRAY: u8 = 0x8;
    pub const LIGHT_BLUE: u8 = 0x9;
    pub const LIGHT_GREEN: u8 = 0xa;
    pub const LIGHT_CYAN: u8 = 0xb;
    pub const LIGHT_RED: u8 = 0xc;
    pub const LIGHT_MAGENTA: u8 = 0xd;
    pub const YELLOW: u8 = 0xe;
    pub const WHITE: u8 = 0xf;

    pub const fn new(front: u8, back: u8) -> Color {
        Self(back << 4 | front)
    }
}

#[derive(Default, Clone, Copy)]
#[repr(C)]
pub struct Char {
    pub code_point: u8,
    pub color: Color,
}

impl Char {
    pub fn new(code_point: u8) -> Self {
        Self {
            color: Color::default(),
            code_point,
        }
    }

    pub fn colored(code_point: u8, color: Color) -> Self {
        Self {
            color,
            code_point,
        }
    }
}

#[repr(transparent)]
pub struct TextBuffer([[Char; Self::WIDTH]; Self::HEIGHT]);

impl TextBuffer {
    // TODO(Dorian): add reference (see discord)
    pub const WIDTH: usize = 80;
    pub const HEIGHT: usize = 25;
    const LOCATION: *mut TextBuffer = 0xB8000 as *mut _;

    pub unsafe fn get() -> &'static Self {
        unsafe {
            &mut *TextBuffer::LOCATION
        }
    }

    pub unsafe fn get_mut() -> &'static mut Self {
        unsafe {
            &mut *TextBuffer::LOCATION
        }
    }
}

impl core::ops::Deref for TextBuffer {
    type Target = [[Char; Self::WIDTH]; Self::HEIGHT];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl core::ops::DerefMut for TextBuffer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
