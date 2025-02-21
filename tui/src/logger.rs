use vga::Color;

use crate::{Rectangle, Screen, Widget};

pub struct Logger;

fn theme(level: log::Level) -> vga::Color {
    match level {
        log::Level::Trace => Color::new(Color::LIGHT_BLUE, Color::BLACK),
        log::Level::Debug => Color::new(Color::LIGHT_GREEN, Color::BLACK),
        log::Level::Info => Color::new(Color::YELLOW, Color::BLACK),
        log::Level::Warn => Color::new(Color::LIGHT_RED, Color::BLACK),
        log::Level::Error => Color::new(Color::RED, Color::BLACK),
    }
}

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
                screen.chars[line][column] = vga::Char {
                    code_point: byte,
                    color: theme(entry.level),
                };
                column += 1;
            }
        }
    }
}
