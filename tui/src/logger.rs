use crate::{Rectangle, Screen, Widget};

pub struct Logger;

impl Widget for Logger
{
    type Event = keyboard::Event;

    fn render(&self, screen: &mut Screen, area: Rectangle) {
        let logger = log::INSTANCE.lock();

        let mut line = (area.y + area.height) as usize;

        for entry in logger.entries().rev().take(area.height as usize) {
            line -= 1;

            let mut column = area.x as usize ;
            // TODO: add text level prefix
            for byte in entry.content.bytes().take(area.width as usize) {
                screen.chars[line][column].code_point = byte;
                column += 1;
            }
        }
    }
}
