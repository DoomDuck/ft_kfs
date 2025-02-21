use collections::ArrayStr;

use crate::Widget;

pub struct TextBuffer {
    keyboard: keyboard::Keyboard,
    content: ArrayStr<{Self::MAX_LEN}>, 
}

impl TextBuffer {
    pub const MAX_LEN: usize = 0x100;

    pub fn new(keyboard: keyboard::Keyboard) -> Self {
        Self {
            keyboard,
            content: ArrayStr::new(),
        }
    }
}


impl Widget for TextBuffer {
    type Event = keyboard::Event;

    fn render(&self, screen: &mut crate::Screen, area: crate::Rectangle) {
        let mut content = self.content.bytes();

        'outer: for line in (0 .. area.height).map(|y| (y + area.y) as usize) {
            for column in (0 .. area.width).map(|x| (x + area.x) as usize) {
                let Some(byte) = content.next() else {
                    break 'outer;
                };
                if byte == b'\n' {
                    break;
                }
                screen.chars[line][column] = vga::Char::new(byte);
            }
        }
    }

    fn update(&mut self, event: Self::Event) {
        if let Some(text) = self.keyboard.feed(event) {
            // Stop writting when out of bounds
            let _ = self.content.push_str(text);
        }
    }
}
