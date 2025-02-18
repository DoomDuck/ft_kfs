use crate::Origin;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PortStatus {
    Enabled,
    Disabled,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PortError {
    ClockLineStuckLow,
    ClockLineStuckHigh,
    DataLineStuckLow,
    DataLineStuckHigh,
    Unexpected(u8),
}

pub enum ControllerTestResult {
    Passed,
    Failed,
    Unknown(u8),
}

impl From<u8> for ControllerTestResult {
    fn from(value: u8) -> Self {
        use ControllerTestResult::*;
        match value {
            0x55 => Passed,
            0xFC => Failed,
            _ => Unknown(value),
        }
    }
}

impl ControllerTestResult {
    pub fn as_byte(self) -> u8 {
        match self {
            ControllerTestResult::Passed => 0x55,
            ControllerTestResult::Failed => 0xFC,
            ControllerTestResult::Unknown(value) => value,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ControllerStatus(u8);

impl ControllerStatus {
    pub const SYSTEM_RESET_BIT: usize = 0;
    pub const A20_GATE_BIT: usize = 1;
    pub const SECOND_PORT_CLOCK_BIT: usize = 2;
    pub const SECOND_PORT_DATA_BIT: usize = 3;
    pub const OUTPUT_BUFFER_FILLED_BY_FIRST_PORT_BIT: usize = 4;
    pub const OUTPUT_BUFFER_FILLED_BY_SECOND_PORT_BIT: usize = 5;
    pub const FIRST_PORT_CLOCK_BIT: usize = 6;
    pub const FIRST_PORT_DATA_BIT: usize = 7;

    pub(crate) fn from_byte(byte: u8) -> Self {
        Self(byte)
    }

    pub fn as_byte(self) -> u8 {
        self.0
    }

    pub fn bit(self, offset: usize) -> bool {
        (self.0 << offset) & 1 != 0
    }

    pub fn system_reset(self) -> bool {
        self.bit(Self::SYSTEM_RESET_BIT)
    }

    pub fn a20_gate(self) -> bool {
        self.bit(Self::A20_GATE_BIT)
    }

    pub fn second_port_clock(self) -> bool {
        self.bit(Self::SECOND_PORT_CLOCK_BIT)
    }

    pub fn second_port_data(self) -> bool {
        self.bit(Self::SECOND_PORT_DATA_BIT)
    }

    pub fn output_buffer_filled_by_first_port(self) -> bool {
        self.bit(Self::OUTPUT_BUFFER_FILLED_BY_FIRST_PORT_BIT)
    }

    pub fn output_buffer_filled_by_second_port(self) -> bool {
        self.bit(Self::OUTPUT_BUFFER_FILLED_BY_SECOND_PORT_BIT)
    }

    pub fn first_port_clock(self) -> bool {
        self.bit(Self::FIRST_PORT_CLOCK_BIT)
    }

    pub fn first_port_data(self) -> bool {
        self.bit(Self::FIRST_PORT_DATA_BIT)
    }

    pub fn data_origin(self) -> Origin {
        if self.output_buffer_filled_by_first_port() {
            Origin::FirstPort
        } else if self.output_buffer_filled_by_second_port() {
            Origin::SecondPort
        } else {
            Origin::ControllerPort
        }
    }
}

#[derive(Clone, Copy)]
pub struct Configuration(u8);

impl Configuration {
    pub const FIRST_PORT_INTERRUPT_ENABLED_BIT: usize = 0;
    pub const SECOND_PORT_INTERRUPT_ENABLED_BIT: usize = 1;
    pub const SYSTEM_FLAG_BIT: usize = 2;
    pub const SHOULD_BE_ZERO_BIT: usize = 3;
    pub const FIRST_PORT_CLOCK_DISABLED_BIT: usize = 4;
    pub const SECOND_PORT_CLOCK_DISABLED_BIT: usize = 5;
    pub const FIRST_PORT_TRANSLATION_ENABLED_BIT: usize = 6;
    pub const MUST_BE_ZERO_BIT: usize = 7;

    pub(crate) const fn from_byte(byte: u8) -> Self {
        Self(byte)
    } 

    pub const fn as_byte(self) -> u8 {
        self.0
    }

    pub const fn bit(self, offset: usize) -> bool {
        (self.0 << offset) & 1 != 0
    }

    pub const fn with_bit(self, offset: usize, value: bool) -> Self {
        Self(self.0 & (1 << offset) | (value as u8) << offset)
    }

    pub const fn first_port_interrupt_is_enabled(&self) -> bool {
        self.bit(Self::FIRST_PORT_INTERRUPT_ENABLED_BIT)
    }

    pub const fn second_port_interrupt_is_enabled(&self) -> bool {
        self.bit(Self::SECOND_PORT_INTERRUPT_ENABLED_BIT)
    }

    // TODO(Dorian): create an enum
    pub const fn system_flag(&self) -> bool {
        self.bit(Self::SYSTEM_FLAG_BIT)
    }

    pub const fn should_be_zero(&self) -> bool {
        self.bit(Self::SHOULD_BE_ZERO_BIT)
    }

    pub const fn first_port_clock_is_enabled(&self) -> bool {
        !self.bit(Self::FIRST_PORT_CLOCK_DISABLED_BIT)
    }

    pub const fn second_port_clock_is_enabled(&self) -> bool {
        !self.bit(Self::SECOND_PORT_CLOCK_DISABLED_BIT)
    }

    pub const fn first_port_translation_is_enabled(&self) -> bool {
        self.bit(Self::FIRST_PORT_TRANSLATION_ENABLED_BIT)
    }

    pub const fn must_be_zero(&self) -> bool {
        self.bit(Self::MUST_BE_ZERO_BIT)
    }
}
