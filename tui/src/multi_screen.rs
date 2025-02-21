use keyboard::{KeyStatus, ScanCode};

use super::Widget;

pub struct MultiScreen<const COUNT: usize, S> {
    current: usize,
    screens: [S; COUNT],
}

impl<const COUNT: usize, S> MultiScreen<COUNT, S> {
    pub const fn new(widgets: [S; COUNT]) -> Self {
        assert!(0 < COUNT, "Cannot create a multi screen with zero screen");
        Self {
            current: 0,
            screens: widgets,
        }
    }

    pub fn previous(&mut self) {
        self.current = (self.current + COUNT - 1) % COUNT;
    }

    pub fn next(&mut self) {
        self.current = (self.current + 1) % COUNT;
    }

    pub fn update(&mut self, event: keyboard::Event) -> Option<keyboard::Event> {
        match event {
            keyboard::Event {
                key_status: KeyStatus::Pressed,
                scan_code: ScanCode::F1,
            } => self.previous(),
            keyboard::Event {
                key_status: KeyStatus::Pressed,
                scan_code: ScanCode::F2,
            } => self.next(),
            not_captured => return Some(not_captured),
        }
        None
    }
}

impl<const COUNT: usize, S> Widget for MultiScreen<COUNT, S>
where
    S: Widget<Event = keyboard::Event>,
{
    type Event = keyboard::Event;

    fn render(&self, screen: &mut crate::Screen, area: crate::Rectangle) {
        self.screens[self.current].render(screen, area);
    }

    fn update(&mut self, event: Self::Event) {
        if let Some(not_captured) = self.update(event) {
            self.screens[self.current].update(not_captured);
        }
    }
}
