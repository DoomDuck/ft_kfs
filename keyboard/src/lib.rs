#![no_std]

pub mod qwerty;
pub mod ergol;

mod scancode;

pub use scancode::ScanCode;

pub struct Event {
    pub scan_code: ScanCode,
    pub key_status: KeyStatus,
}

#[derive(Debug, Clone, Copy)]
pub enum KeyStatus {
    Pressed,
    Released,
}

pub struct Keyboard {
    keymap: Keymap,
    current_layer: LayerId,
    key_statuses: [KeyStatus; ScanCode::COUNT],
}

impl Keyboard {
    pub const fn qwerty() -> Self {
        Self {
            keymap: qwerty::KEYMAP,
            current_layer: LayerId::One,
            key_statuses: [KeyStatus::Released; ScanCode::COUNT],
        }
    }

    pub const fn ergol() -> Self {
        Self {
            keymap: ergol::KEYMAP,
            current_layer: LayerId::One,
            key_statuses: [KeyStatus::Released; ScanCode::COUNT],
        }
    }

    pub fn feed(&mut self, event: Event) -> Option<&'static str> {
        let Event {
            scan_code,
            key_status,
        } = event;
        self.key_statuses[scan_code as usize] = key_status;
        let effect = self.keymap[self.current_layer].effects[scan_code as usize];
        match (effect, key_status) {
            (Effect::Emit(text), KeyStatus::Pressed) => Some(text),
            (Effect::OnPressGotoLayer(layer_id), KeyStatus::Pressed)
            | (Effect::OnReleaseGotoLayer(layer_id), KeyStatus::Released) => {
                self.current_layer = layer_id;
                None
            }
            _ => None,
        }
    }
}

pub struct Keymap {
    name: &'static str,
    layers: [Layer; LayerId::COUNT],
}

impl core::ops::Index<LayerId> for Keymap {
    type Output = Layer;

    fn index(&self, id: LayerId) -> &Self::Output {
        &self.layers[id as usize]
    }
}

impl core::ops::IndexMut<LayerId> for Keymap {
    fn index_mut(&mut self, id: LayerId) -> &mut Self::Output {
        &mut self.layers[id as usize]
    }
}

#[repr(u8)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LayerId {
    #[default]
    One,
    Two,
    Three,
    Four,
}

impl LayerId {
    pub const COUNT: usize = 4;
}

pub struct Layer {
    name: &'static str,
    effects: [Effect; ScanCode::COUNT],
}

impl Layer {
    const UNUSED: Layer = Layer::new("unused", &[]);

    const fn new(name: &'static str, mapping: &[(ScanCode, Effect)]) -> Self {
        let mut effects = [Effect::Ignore; ScanCode::COUNT];
        let mut index = 0;
        while index < mapping.len() {
            let (scan_code, effect) = mapping[index];
            effects[scan_code as usize] = effect;
            index += 1;
        }
        Self { name, effects }
    }
}

#[derive(Debug, Clone, Copy)]
enum Effect {
    Ignore,
    Emit(&'static str),
    OnPressGotoLayer(LayerId),
    OnReleaseGotoLayer(LayerId),
}
