#![no_std]
#![no_main]

extern crate cortex_m;
extern crate cortex_m_semihosting;
extern crate embedded_hal;
extern crate feather_m0 as hal;
#[cfg(not(feature = "use_semihosting"))]
extern crate panic_halt;
#[cfg(feature = "use_semihosting")]
extern crate panic_semihosting;

use hal::clock::GenericClockController;
use hal::delay::Delay;
use hal::prelude::*;
use hal::{entry, CorePeripherals, Peripherals};

struct OLED<SPI, CS, DC> {
    spi: SPI,
    cs: CS,
    dc: DC,
}

impl<SPI, CS, DC> OLED<SPI, CS, DC>
where
    SPI: embedded_hal::spi::FullDuplex<u8>,
    CS: embedded_hal::digital::v2::OutputPin,
    DC: embedded_hal::digital::v2::OutputPin,
{
    pub fn new(spi: SPI, cs: CS, dc: DC) -> Self {
        return Self {
            spi: spi,
            cs: cs,
            dc: dc,
        };
    }
}

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::with_external_32kosc(
        peripherals.GCLK,
        &mut peripherals.PM,
        &mut peripherals.SYSCTRL,
        &mut peripherals.NVMCTRL,
    );
    let mut pins = hal::Pins::new(peripherals.PORT);
    let mut red_led = pins.d13.into_open_drain_output(&mut pins.port);
    let mut delay = Delay::new(core.SYST, &mut clocks);

    let spi_dev = hal::spi_master(
        &mut clocks,
        hal::time::MegaHertz(48),
        peripherals.SERCOM4,
        &mut peripherals.PM,
        pins.sck,
        pins.mosi,
        pins.miso,
        &mut pins.port,
    );
    let _oled = OLED::new(
        spi_dev,
        pins.d5.into_open_drain_output(&mut pins.port),
        pins.d6.into_open_drain_output(&mut pins.port),
    );

    loop {
        delay.delay_ms(255u8);
        red_led.set_high();
        delay.delay_ms(255u8);
        red_led.set_low();
    }
}
