#![no_std]

use bcdtime::DateTime;
use graphics::vector::{Rect, Vector};

pub mod blockfont;
pub mod digitfont;
pub mod gfx;
pub mod minifont;

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
        gfx::draw_big_digit(self.datetime.hour.tens() as u8, disp, Vector(0, 0)).unwrap();
        gfx::draw_big_digit(self.datetime.hour.units() as u8, disp, Vector(10, 0)).unwrap();
        gfx::draw_big_digit(self.datetime.minute.tens() as u8, disp, Vector(24, 0)).unwrap();
        gfx::draw_big_digit(self.datetime.minute.units() as u8, disp, Vector(34, 0)).unwrap();
        if self.colon {
            disp.fill_rect(Rect::new4(20, 5, 22, 7)).unwrap();
            disp.fill_rect(Rect::new4(20, 9, 22, 11)).unwrap();
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
