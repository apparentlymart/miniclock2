#![no_std]
#![feature(const_str_as_bytes)]

use bcdtime::DateTime;
use graphics::vector::Vector;

pub mod blockfont;
pub mod digitfont;
pub mod gfx;
pub mod minifont;
pub mod tiles;

const WEEKDAY: [&'static [u8]; 7] = [
    "MON".as_bytes(),
    "TUE".as_bytes(),
    "WED".as_bytes(),
    "THU".as_bytes(),
    "FRI".as_bytes(),
    "SAT".as_bytes(),
    "SUN".as_bytes(),
];

pub struct App<Clock, Display>
where
    Clock: bcdtime::Read + bcdtime::Write,
    Display: graphics::Display,
{
    clock: Clock,
    display: Display,
    colon: bool,
    datetime: DateTime,
}

impl<Clock, Display> App<Clock, Display>
where
    Clock: bcdtime::Read + bcdtime::Write,
    Display: graphics::Display,
{
    pub fn new(mut clock: Clock, display: Display) -> Self {
        let init_time = clock.read().unwrap();

        Self {
            clock: clock,
            display: display,
            colon: false,
            datetime: init_time,
        }
    }

    // Advance the app's state machine based on events detected since the
    // last call.
    pub fn update(&mut self, evts: &Events) {
        if evts.tick {
            self.colon = !self.colon;
            self.datetime = self.clock.read().unwrap();
        }
    }

    pub fn redraw(&mut self) {
        let disp = &mut self.display;

        disp.clear().unwrap();

        {
            gfx::draw_big_digit(self.datetime.hour.tens() as u8, disp, Vector(0, 0)).unwrap();
            gfx::draw_big_digit(self.datetime.hour.units() as u8, disp, Vector(10, 0)).unwrap();
            gfx::draw_big_digit(self.datetime.minute.tens() as u8, disp, Vector(24, 0)).unwrap();
            gfx::draw_big_digit(self.datetime.minute.units() as u8, disp, Vector(34, 0)).unwrap();
            if self.colon {
                gfx::draw_colon(disp, Vector(20, 0)).unwrap();
            }
        }

        {
            let weekday = self.datetime.day as usize;
            gfx::draw_block_text(&WEEKDAY[weekday][..], disp, Vector(47, 5)).unwrap();
        }

        {
            let date = self.datetime.date;
            let units = date.units();
            let tens = date.tens();
            let ordinal_lig = if tens == 1 {
                0x7b // th
            } else {
                match units {
                    1 => 0x7c, // st
                    2 => 0x7d, // nd
                    3 => 0x7e, // rd
                    _ => 0x7b, // th
                }
            };
            let ordinal_lig_wid = match ordinal_lig {
                0x7b => 6, // th
                0x7d => 6, // nd
                _ => 5,
            };
            gfx::draw_block_char(
                '0' as u8 + tens as u8,
                disp,
                Vector(64 - ordinal_lig_wid - 12, 11),
            )
            .unwrap();
            gfx::draw_block_char(
                '0' as u8 + units as u8,
                disp,
                Vector(64 - ordinal_lig_wid - 6, 11),
            )
            .unwrap();
            gfx::draw_block_char(ordinal_lig, disp, Vector(64 - ordinal_lig_wid, 11)).unwrap();
        }

        self.display.flip().unwrap();
    }
}

pub struct Events {
    pub tick: bool,
}

impl Events {
    pub fn reset(&mut self) {
        *self = Self::default();
    }

    pub fn has_pending(&self) -> bool {
        *self != Self::default()
    }
}

impl core::default::Default for Events {
    fn default() -> Self {
        Self { tick: false }
    }
}

impl core::cmp::PartialEq<Events> for Events {
    fn eq(&self, other: &Events) -> bool {
        self.tick == other.tick
    }
}
