#![no_std]
use vga::Char;

mod multi_screen;
mod logger;

pub use multi_screen::MultiScreen;

pub struct Screen {
    chars: [[Char; Self::WIDTH]; Self::HEIGHT],
}

impl Screen {
    pub const WIDTH: usize = vga::TextBuffer::WIDTH;
    pub const HEIGHT: usize = vga::TextBuffer::HEIGHT;
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Rectangle {
    x: u16,
    y: u16,
    width: u16,
    height: u16,
}

pub trait Widget {
    type Event;

    fn render(&self, screen: &mut Screen, area: Rectangle);

    fn update(&mut self, _event: Self::Event) { }
}
