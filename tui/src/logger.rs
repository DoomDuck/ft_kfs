use crate::Widget;

pub struct Logger;

impl Widget for Logger
{
    type Event = keyboard::Event;

    fn render(&self, screen: &mut crate::Screen, area: crate::Rectangle) {
        // log::INSTANCE.lock()
    }

    fn update(&mut self, event: Self::Event) {
        todo!()
    }
}
