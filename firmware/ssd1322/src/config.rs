pub enum DisplayMode {
    AllOff,
    AllOn,
    Normal,
    Inverted,
}

pub struct Function {
    pub internal_vdd_reg: bool,
}

pub enum GPIOState {
    HiZ(bool),
    Output(bool),
}

impl GPIOState {
    pub(crate) fn protocol_arg(&self) -> u8 {
        match self {
            GPIOState::HiZ(input) => {
                if *input {
                    0b01
                } else {
                    0b00
                }
            }
            GPIOState::Output(high) => {
                if *high {
                    0b11
                } else {
                    0b10
                }
            }
        }
    }
}

pub struct DisplayEnhancementA {
    pub vsl: VSL,
    pub low_gs_quality: LowGSQuality,
}

impl DisplayEnhancementA {
    pub(crate) fn protocol_args(&self) -> (u8, u8) {
        let mut a = 0b10100000;
        let mut b = 0b00000101;

        if let VSL::Internal = self.vsl {
            a |= 0b10 << 0;
        }
        match self.low_gs_quality {
            LowGSQuality::Normal => b |= 0b10110 << 3,
            LowGSQuality::Enhanced => b |= 0b11111 << 3,
        }

        (a, b)
    }
}

pub enum DisplayEnhancementB {
    Normal,
    Reserved,
}

impl DisplayEnhancementB {
    pub(crate) fn protocol_args(&self) -> (u8, u8) {
        let a = if let DisplayEnhancementB::Normal = self {
            0b10100010
        } else {
            0b10000010
        };
        let b = 0b00100000;
        (a, b)
    }
}

pub enum VSL {
    Internal,
    External,
}

pub enum LowGSQuality {
    Normal,
    Enhanced,
}

pub struct Remap {
    pub address_increment: WriteDirection,
    pub column_addr_remap: bool,
    pub nibble_remap: bool,
    pub scan_direction: ScanDirection,
    pub com_split_odd_even: bool,
    pub dual_com_mode: bool,
}

pub enum WriteDirection {
    Horizontal,
    Vertical,
}

pub enum ScanDirection {
    Forward,
    Backward,
}

impl Remap {
    pub(crate) fn protocol_args(&self) -> (u8, u8) {
        let mut a = 0b00000000;
        let mut b = 0b00000001;

        if let WriteDirection::Vertical = self.address_increment {
            a |= 0b1 << 0;
        }
        if self.column_addr_remap {
            a |= 0b1 << 1;
        }
        if self.nibble_remap {
            a |= 0b1 << 2;
        }
        if let ScanDirection::Backward = self.scan_direction {
            a |= 0b1 << 4;
        }
        if self.com_split_odd_even {
            a |= 0b1 << 5;
        }
        if self.dual_com_mode {
            b |= 0b1 << 4;
        }

        (a, b)
    }
}
