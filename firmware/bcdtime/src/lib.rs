#![no_std]

/// A date and time represented with BCD numbers.
pub struct DateTime {
    pub second : BCD,
    pub minute : BCD,
    pub hour: BCD,
    pub day: u8,
    pub date: BCD,
    pub month: BCD,
    pub year: BCD,
    pub hr24: bool,
}

/// Implemented by objects that can read `DateTime` values.
pub trait Read {
    type Error;

    fn read(&self) -> Result<DateTime, Self::Error>;
}

/// Implemented by objects that can write `DateTime` values.
pub trait Write {
    type Error;

    fn write(&mut self, dt: &DateTime) -> Result<(), Self::Error>;
}

/// A number between 0 and 99, in binary-coded decimal.
#[derive(Copy, Debug, Clone)]
pub struct BCD(u8);

impl BCD {
    pub fn tens(self) -> i32 {
        (self.0 >> 4) as i32
    }

    pub fn units(self) -> i32 {
        (self.0 & 0xf) as i32
    }

    pub fn digit(self, idx: usize) -> i32 {
        ((self.0 >> (4 * idx)) & 0xf) as i32
    }
}

impl core::convert::Into<u8> for BCD {
    fn into(self) -> u8 {
        (self.0 >> 4) * 10 + self.0 & 0xf
    }
}

impl core::convert::From<u8> for BCD {
    fn from(v: u8) -> Self {
        Self(((v / 10) << 4) | (v % 10))
    }
}
