#![no_std]
use vga::{Char, Color};

mod logger;
mod multi_screen;
mod text_buffer;

pub use logger::Logger;
pub use multi_screen::MultiScreen;
pub use text_buffer::TextBuffer;

pub struct Screen {
    pub chars: [[Char; Self::WIDTH]; Self::HEIGHT],
}

impl Screen {
    pub const WIDTH: usize = vga::TextBuffer::WIDTH;
    pub const HEIGHT: usize = vga::TextBuffer::HEIGHT;

    pub fn area(&self) -> Rectangle {
        Rectangle {
            x: 0,
            y: 0,
            width: Self::WIDTH as u16,
            height: Self::HEIGHT as u16,
        }
    }

    pub fn clear(&mut self) {
        self.chars = [[Char::new(0); Self::WIDTH]; Self::HEIGHT]
    }

    pub unsafe fn write_to_vga(&self) {
        vga::TextBuffer::get_mut()
            .copy_from_slice(&self.chars);
    }
}

impl Default for Screen {
    fn default() -> Self {
        Self {
            chars: [[const {
                Char {
                    code_point: 0,
                    color: Color::new(Color::WHITE, Color::BLACK),
                }
            }; Self::WIDTH]; Self::HEIGHT],
        }
    }
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

    fn update(&mut self, _event: Self::Event) {}
}

impl<E> Widget for &mut dyn Widget<Event = E> {
    type Event = E;

    fn render(&self, screen: &mut Screen, area: Rectangle) {
        (self as &dyn Widget<Event = E>).render(screen, area);
    }

    fn update(&mut self, event: Self::Event) {
        (self as &mut dyn Widget<Event = E>).update(event);
    }
}
