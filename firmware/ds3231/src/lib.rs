#![no_std]
#![feature(type_alias_enum_variants)]

use embedded_hal::blocking::i2c;
use bcdtime::{BCD, DateTime};

const I2C_ADDR: u8 = 0b1101000;

pub struct DS3231<I2C>
where
    I2C: i2c::WriteRead + i2c::Write,
{
    i2c: I2C,
}

impl<I2C, WRErr, WErr> DS3231<I2C>
where
    I2C: i2c::WriteRead<Error=WRErr> + i2c::Write<Error=WErr>,
    WRErr: core::fmt::Debug,
    WErr: core::fmt::Debug,
{
    pub fn new(i2c: I2C) -> Self {
        return Self {
            i2c: i2c,
        }
    }

    pub fn set_square_wave(&mut self, freq: SquareWaveFrequency) -> Result<(), Error<WRErr, WErr>> {
        // FIXME: This currently clobbers a bunch of other settings in
        // this same register. Should do a read/modify/write instead.

        let raw = match freq {
            SquareWaveFrequency::Disabled => {
                self.i2c.write(I2C_ADDR, &[0x0eu8, 0b00000100u8]).map_err(Error::w)?;
                return Ok(());
            },
            SquareWaveFrequency::Freq1Hz => 0b00000000,
            SquareWaveFrequency::Freq1_024kHz => 0b00001000,
            SquareWaveFrequency::Freq4_096kHz => 0b00010000,
            SquareWaveFrequency::Freq8_192kHz => 0b00011000,
        } as u8;
        self.i2c.write(I2C_ADDR, &[0x0eu8, raw]).map_err(Error::w)
    }
}

impl<I2C, WRErr, WErr> bcdtime::Read for DS3231<I2C>
where
    I2C: i2c::WriteRead<Error=WRErr> + i2c::Write<Error=WErr>,
    WRErr: core::fmt::Debug,
    WErr: core::fmt::Debug,
{
    type Error = Error<WRErr, WErr>;

    fn read(&mut self) -> Result<DateTime, Self::Error> {
        let mut result: [u8; 7] = [0u8; 7];
        self.i2c.write_read(I2C_ADDR, &[0u8], &mut result[..]).map_err(Error::wr)?;

        Ok(DateTime {
            second: BCD::from_raw(result[0]),
            minute: BCD::from_raw(result[1]),
            hour: {
                // The hour part has the 24 hour flag packed into it too,
                // so we need to mask it off. Additionally, if we're in 12-hour
                // mode then bit 5 is the AM/PM indicator rather than a BCD
                // digit, so we'll need to strip it. (Note that the AM/PM
                // indicator isn't exposed anywhere, because 12-hour time is
                // odd and I don't really care about it.)
                if (result[2] & 0b01000000) == 0 {
                    BCD::from_raw(result[2] & 0b00111111)
                } else {
                    BCD::from_raw(result[2] & 0b00011111)
                }
            },
            day: result[3],
            date: BCD::from_raw(result[4]),
            month: {
                // The month part has the century flag packed into it too,
                // so we need to mask it off.
                BCD::from_raw(result[5] & 0b00011111)
            },
            year: BCD::from_raw(result[6]),
            hr24: {
                // 24-hour flag is packed in to the hour field.
                (result[2] & 0b01000000) == 0
            }
        })
    }
}

impl<I2C, WRErr, WErr> bcdtime::Write for DS3231<I2C>
where
    I2C: i2c::WriteRead<Error=WRErr> + i2c::Write<Error=WErr>,
    WRErr: core::fmt::Debug,
    WErr: core::fmt::Debug,
{
    type Error = Error<WRErr, WErr>;

    fn write(&mut self, dt: &DateTime) -> Result<(), Self::Error> {
        let mut raw: [u8; 8] = [0u8; 8];
        let data = &mut raw[1..]; // skip the first byte, which is the register address

        data[0] = dt.second.raw();
        data[1] = dt.minute.raw();
        data[2] = dt.hour.raw();
        data[3] = dt.day;
        data[4] = dt.date.raw();
        data[5] = dt.month.raw(); // NOTE: This always sets century back to zero
        data[6] = dt.year.raw();

        if !dt.hr24 {
            // Need to also pack the 12-hour-time flag into the hour field.
            data[2] = data[2] & (1 << 6);
        }

        self.i2c.write(I2C_ADDR, &raw[..]).map_err(Error::w)?;

        Ok(())
    }
}

#[derive(Debug)]
pub enum Error<WRErr, WErr>
where
    WRErr: core::fmt::Debug,
    WErr: core::fmt::Debug,
{
    Request,
    Protocol,
    WriteRead(WRErr),
    Write(WErr),
}

impl<WRErr, WErr> Error<WRErr, WErr>
where
    WRErr: core::fmt::Debug,
    WErr: core::fmt::Debug,
{
    fn wr(err: WRErr) -> Self {
        Self::WriteRead(err)
    }

    fn w(err: WErr) -> Self {
        Self::Write(err)
    }
}

pub enum SquareWaveFrequency {
    Disabled,
    Freq1Hz,
    Freq1_024kHz,
    Freq4_096kHz,
    Freq8_192kHz,
}
