#![no_std]

pub mod keyboard;
pub mod controller;

use core::fmt;

use controller::{Configuration, ControllerStatus, ControllerTestResult, PortError};

pub struct Controller {
    pub data_port: port::Handle<{ Self::DATA_PORT_ID }>,
    pub controller_port: port::Handle<{ Self::CONTROLLER_PORT_ID }>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Origin {
    FirstPort,
    SecondPort,
    ControllerPort,
}

impl Controller {
    pub const DATA_PORT_ID: port::ID = 0x60;
    pub const CONTROLLER_PORT_ID: port::ID = 0x64;

    fn status(&mut self) -> Status {
        Status(self.controller_port.read_u8())
    }

    fn run(&mut self, command: Command) {
        self.controller_port.write_u8(command.as_byte())
    }

    fn read_ram(&mut self, address: RamAddress) -> u8 {
        self.run(Command::ReadRAM(address));
        self.data_port.read_u8()
    }

    fn write_ram(&mut self, address: RamAddress, value: u8) {
        self.run(Command::WriteRAM(address));
        self.data_port.write_u8(value);
    }

    pub fn configuration(&mut self) -> Configuration {
        let byte = self.read_ram(RamAddress::CONTROLLER_CONFIGURATION);
        Configuration::from_byte(byte)
    }

    pub fn set_configuration(&mut self, configuration: Configuration) {
        self.write_ram(
            RamAddress::CONTROLLER_CONFIGURATION,
            configuration.as_byte(),
        );
    }

    fn ready_for_write(&mut self) -> bool {
        !self.status().output_buffer_is_full() 
    }

    fn try_read_n_times_no_origin(&mut self, n: usize) -> Option<u8> {
        for _ in 0..n {
            if let Ok(byte) = self.direct_read_without_origin() {
                return Some(byte)
            }
        }
        None
    }

    fn direct_read(&mut self) -> Result<(Origin, u8), ()> {
        match self.status().output_buffer_is_full() {
            true => {
                let byte = self.data_port.read_u8();
                let origin = self.controller_status().data_origin();
                Ok((origin, byte))
            }
            false => Err(()),
        }
    }

    fn direct_read_without_origin(&mut self) -> Result<u8, ()> {
        match self.status().output_buffer_is_full() {
            true => Ok(self.data_port.read_u8()),
            false => Err(()),
        }
    }

    pub fn read(&mut self) -> (Origin, u8) {
        loop {
            match self.direct_read() {
                Ok(value) => return value,
                Err(()) => core::hint::spin_loop(),
            }
        }
    }

    pub fn read_without_origin(&mut self) -> u8 {
        loop {
            match self.direct_read_without_origin() {
                Ok(value) => return value,
                Err(()) => core::hint::spin_loop(),
            }
        }
    }

    fn direct_write(&mut self, value: u8) -> Result<(), ()> {
        match self.ready_for_write() {
            true => Ok(self.data_port.write_u8(value)),
            false => Err(()),
        }
    }

    fn write(&mut self, value: u8) {
        loop {
            match self.direct_write(value) {
                Ok(()) => break,
                Err(()) => core::hint::spin_loop(),
            }
        }
    }

    pub fn flush(&mut self) {
        // Read all directly available bytes
        while let Ok(_) = self.direct_read_without_origin() {}
    }

    pub fn disable_second_port(&mut self) {
        self.run(Command::DisableSecondPort)
    }

    pub fn enable_second_port(&mut self) {
        self.run(Command::EnableSecondPort)
    }

    pub fn test_second_port(&mut self) -> Result<(), PortError> {
        self.run(Command::TestSecondPort);
        let byte = self.data_port.read_u8();
        decode_port_test_result(byte)
    }

    pub fn test_controller(&mut self) -> ControllerTestResult {
        self.run(Command::TestController);
        let byte = self.data_port.read_u8();
        ControllerTestResult::from(byte)
    }

    pub fn test_first_port(&mut self) -> Result<(), PortError> {
        self.run(Command::TestFirstPort);
        let byte = self.data_port.read_u8();
        decode_port_test_result(byte)
    }

    pub fn disable_first_port(&mut self) {
        self.run(Command::DisableFirstPort)
    }

    pub fn enable_first_port(&mut self) {
        self.run(Command::EnableFirstPort)
    }

    pub fn controller_status(&mut self) -> ControllerStatus {
        self.run(Command::ReadControllerOutputPort);
        let byte = self.data_port.read_u8();
        ControllerStatus::from_byte(byte)
    }

    pub fn set_controller_status(&mut self, status: ControllerStatus) {
        self.run(Command::WriteToControllerOutputPort);
        self.write(status.as_byte())
    }

    pub fn write_to_first_port(&mut self, value: u8) {
        self.write(value)
    }

    pub fn write_to_second_port(&mut self, value: u8) {
        self.run(Command::WriteToSecondPortInput);
        self.write(value)
    }

    pub fn set_second_port_output(&mut self, value: u8) {
        self.run(Command::WriteToSecondPortOutput);
        self.write(value);
    }

    pub fn reset_controller(&mut self) {
        self.run(Command::PulseOutputLines {
            reset: true,
            unknown_1: false,
            unknown_2: false,
            unknown_3: false,
        });
    }

    pub fn initialize(&mut self) -> Result<(Option<Type>, Option<Type>), ()> {
        // Disable first and second port
        self.disable_first_port();
        self.disable_second_port();

        // Flush unique byte data buffer
        self.flush();

        log::log!("Aquiring config...\n");

        // Setup controller configuration byte
        let controller_configuration = self.configuration();

        let new_configuration = controller_configuration
            .with_bit(Configuration::FIRST_PORT_INTERRUPT_ENABLED_BIT, false)
            .with_bit(Configuration::FIRST_PORT_CLOCK_DISABLED_BIT, false)
            .with_bit(Configuration::FIRST_PORT_TRANSLATION_ENABLED_BIT, false);


        log::log!("Setting config\n");
        self.set_configuration(new_configuration);

        log::log!("Enabling second port\n");
        // Determin the number of port
        self.enable_second_port();

        let is_dual_channel = self.configuration().second_port_clock_is_enabled();
        if is_dual_channel {
            log::log!("Is dual channel\n");
            // Disable what has been activated
            self.disable_second_port();

            let new_configuration = self
                .configuration()
                .with_bit(Configuration::SECOND_PORT_CLOCK_DISABLED_BIT, false)
                .with_bit(Configuration::SECOND_PORT_CLOCK_DISABLED_BIT, false);

            self.set_configuration(new_configuration);
        }

        // Test first port
        let first_port_is_ok = self.test_first_port().is_ok();

        let second_port_is_ok = is_dual_channel && self.test_second_port().is_ok();

        let has_working_port = first_port_is_ok || second_port_is_ok;

        if !has_working_port {
            return Err(());
        }

        log::log!("Has working ports\n");

        if first_port_is_ok {
            log::log!("first_port_is_ok\n");
            self.enable_first_port();
        }

        // TODO: Decide what to do
        // if second_port_is_ok {
        //     log::log!("second_port_is_ok\n");
        //     self.enable_second_port();
        // }

        let mut first_device_type = None;
        if first_port_is_ok {
            self.write_to_first_port(0xFF);

            let byte_1 = self.read_without_origin();
            let byte_2 = self.read_without_origin();
            let mut response = (byte_1, byte_2);

            if response.0 < response.1 {
                core::mem::swap(&mut response.0, &mut response.1);
            }

            log::log!("response: {:x} {:x}\n", response.0, response.1);

            if response == (0xFA, 0xAA) {
                first_device_type = Some(match self.try_read_n_times_no_origin(0x1000) {
                    None => Type::AncientATKeyboard,
                    Some(0x00) => Type::StandardPS2Mouse,
                    Some(0x03) => Type::MouseWithScrollWheel,
                    Some(0x04) => Type::FiveButtonMouse,
                    Some(0xAB) => match self.read_without_origin() {
                        0x83 | 0xC1 => Type::MF2Keyboard,
                        value => Type::Unknown(0xAB, value),
                    },
                    Some(0xAC) => match self.read_without_origin() {
                        value => Type::Unknown(0xAC, value),
                    },
                    Some(value) => Type::Unknown(0x00, value),
                });
            }
        }

        let mut second_device_type = None;
        // if second_port_is_ok {
        //     self.write_to_second_port(0xFF);
        //
        //     let byte_1 = self.read_without_origin();
        //     let byte_2 = self.read_without_origin();
        //     let mut response = (byte_1, byte_2);
        //
        //     if response.0 < response.1 {
        //         core::mem::swap(&mut response.0, &mut response.1);
        //     }
        //
        //     log::log!("response: {:x} {:x}\n", response.0, response.1);
        //
        //     if response == (0xFA, 0xAA) {
        //         // second_device_type = Some(match self.read_without_origin() {
        //         second_device_type = Some(match self.try_read_n_times_no_origin(0x1000) {
        //             None => Type::AncientATKeyboard,
        //             Some(0x00) => Type::StandardPS2Mouse,
        //             Some(0x03) => Type::MouseWithScrollWheel,
        //             Some(0x04) => Type::FiveButtonMouse,
        //             Some(0xAB) => match self.read_without_origin() {
        //                 0x83 | 0xC1 => Type::MF2Keyboard,
        //                 value => Type::Unknown(0xAB, value),
        //             },
        //             Some(0xAC) => match self.read_without_origin() {
        //                 value => Type::Unknown(0xAC, value),
        //             },
        //             Some(value) => Type::Unknown(0x00, value),
        //         });
        //     }
        // }

        Ok((first_device_type, second_device_type))
    }
}

// TODO(Dorian): Complete enum
#[non_exhaustive]
#[derive(Debug)]
pub enum Type {
    AncientATKeyboard,
    StandardPS2Mouse,
    MouseWithScrollWheel,
    FiveButtonMouse,
    MF2Keyboard,
    Unknown(u8, u8),
}

fn decode_port_test_result(result: u8) -> Result<(), PortError> {
    match result {
        0x00 => Ok(()),
        0x01 => Err(PortError::ClockLineStuckLow),
        0x02 => Err(PortError::ClockLineStuckHigh),
        0x03 => Err(PortError::DataLineStuckLow),
        0x04 => Err(PortError::DataLineStuckHigh),
        _ => Err(PortError::Unexpected(result)),
    }
}

#[derive(Debug, Clone, Copy)]
struct RamAddressOutOfRange(u8);

impl fmt::Display for RamAddressOutOfRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Ram address {:#x} is out of range", self.0)
    }
}

impl core::error::Error for RamAddressOutOfRange {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RamAddress(u8);

impl RamAddress {
    pub const ZERO: Self = Self(0);
    pub const CONTROLLER_CONFIGURATION: Self = Self::ZERO;
    pub const RANGE: core::ops::Range<u8> = 0..0x40;

    /// # Panic
    /// When address is out of [Self::RANGE]
    fn new(address: u8) -> Self {
        Self::try_new(address).unwrap()
    }

    fn try_new(address: u8) -> Result<Self, RamAddressOutOfRange> {
        match Self::RANGE.contains(&address) {
            true => Ok(Self(address)),
            false => Err(RamAddressOutOfRange(address)),
        }
    }
}

/// Based of [OSDev.org](https://wiki.osdev.org/%228042%22_PS/2_Controller#PS/2_Controller_Commands)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Command {
    // 0x20 to 0x3f
    ReadRAM(RamAddress),
    // 0x60 to 0x7f
    WriteRAM(RamAddress),

    // 0xA7
    DisableSecondPort,
    // 0xA8
    EnableSecondPort,

    // 0xA9
    TestSecondPort,
    // 0xAA
    TestController,
    // 0xAB
    TestFirstPort,

    // 0xAC
    DiagnoticDump,

    // 0xAD
    DisableFirstPort,
    // 0xAE
    EnableFirstPort,

    // 0xC0
    ReadControllerInputPort,

    // 0xC1
    SetStatusSecondNibbleFromInputPortFirstNibble,
    // 0xC2
    SetStatusSecondNibbleFromInputPortSecondNibble,

    // 0xD0
    ReadControllerOutputPort,

    // 0xD1
    WriteToControllerOutputPort,

    // 0xD2
    WriteToFirstPortOutput,

    // 0xD3
    WriteToSecondPortOutput,

    // 0xD4
    WriteToSecondPortInput,

    // 0xF0 to 0xFF
    PulseOutputLines {
        reset: bool,
        unknown_1: bool,
        unknown_2: bool,
        unknown_3: bool,
    },
}

impl Command {
    fn as_byte(self) -> u8 {
        use Command::*;
        match self {
            ReadRAM(address) => 0x20 + address.0,
            WriteRAM(address) => 0x60 + address.0,

            DisableSecondPort => 0xA7,
            EnableSecondPort => 0xA8,

            TestSecondPort => 0xA9,
            TestController => 0xAA,
            TestFirstPort => 0xAB,

            DiagnoticDump => 0xAC,

            DisableFirstPort => 0xAD,
            EnableFirstPort => 0xAE,

            ReadControllerInputPort => 0xC0,

            SetStatusSecondNibbleFromInputPortFirstNibble => 0xC1,
            SetStatusSecondNibbleFromInputPortSecondNibble => 0xC2,

            ReadControllerOutputPort => 0xD0,
            WriteToControllerOutputPort => 0xD1,
            WriteToFirstPortOutput => 0xD2,
            WriteToSecondPortOutput => 0xD3,
            WriteToSecondPortInput => 0xD4,

            PulseOutputLines {
                reset,
                unknown_1,
                unknown_2,
                unknown_3,
            } => {
                // 0 => pulse
                // 1 => don't pulse
                let pulsing_output_lines_mask = (reset as u8) << 0
                    | (unknown_1 as u8) << 1
                    | (unknown_2 as u8) << 2
                    | (unknown_3 as u8) << 3;
                0xf0 | pulsing_output_lines_mask
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum CommandTarget {
    Device,
    Controller,
}

impl CommandTarget {
    pub fn is_device(self) -> bool {
        self == CommandTarget::Device
    }

    pub fn is_controller(self) -> bool {
        self == CommandTarget::Controller
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Status(u8);

impl Status {
    pub const PORT: port::ID = 0x64;
    pub const OUTPUT_BUFFER_IS_FULL_BIT: usize = 0;
    pub const INPUT_BUFFER_IS_FULL_BIT: usize = 1;
    pub const SYSTEM_FLAG_BIT: usize = 2;
    pub const COMMAND_TARGET_IS_CONTROLLER_BIT: usize = 3;
    pub const MAYBE_KEYBOARD_LOCK_BIT: usize = 4;
    pub const MAYBE_RECEIVE_TIMEOUT_BIT: usize = 5;
    pub const TIMEOUT_ERROR_BIT: usize = 6;
    pub const PARITY_ERROR_BIT: usize = 7;

    pub fn as_byte(self) -> u8 {
        self.0
    }

    pub fn bit(self, offset: usize) -> bool {
        (self.0 << offset) & 1 != 0
    }

    pub fn output_buffer_is_full(&self) -> bool {
        self.bit(Self::OUTPUT_BUFFER_IS_FULL_BIT)
    }

    pub fn input_buffer_is_full(&self) -> bool {
        self.bit(Self::INPUT_BUFFER_IS_FULL_BIT)
    }

    pub fn system_flag(&self) -> bool {
        self.bit(Self::SYSTEM_FLAG_BIT)
    }

    pub fn command_target(&self) -> CommandTarget {
        match self.bit(Self::COMMAND_TARGET_IS_CONTROLLER_BIT) {
            false => CommandTarget::Device,
            true => CommandTarget::Controller,
        }
    }

    pub fn maybe_keyboard_lock(&self) -> bool {
        self.bit(Self::MAYBE_KEYBOARD_LOCK_BIT)
    }

    pub fn maybe_receive_timeout(&self) -> bool {
        self.bit(Self::MAYBE_RECEIVE_TIMEOUT_BIT)
    }

    pub fn timeout_error(&self) -> bool {
        self.bit(Self::TIMEOUT_ERROR_BIT)
    }

    pub fn parity_error(&self) -> bool {
        self.bit(Self::PARITY_ERROR_BIT)
    }
}
