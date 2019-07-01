#![no_std]
#![no_main]

// We don't have enough flash space for all of the usual panic formatting
// machinery, so we'll just halt on panic.
extern crate panic_halt;

use cortex_m::asm;
use cortex_m_rt::entry;
use lpc81x_pac;

#[entry]
fn main() -> ! {
    let p = lpc81x_pac::Peripherals::take().unwrap();

    p.GPIO_PORT
        .dir0
        .modify(|r, w| unsafe { w.bits(r.bits() | 1 << 17) });
    p.GPIO_PORT
        .dir0
        .modify(|r, w| unsafe { w.bits(r.bits() | 1 << 17) });

    loop {
        p.GPIO_PORT.not0.write(|w| unsafe { w.bits(1 << 17) });
        for _ in 0..1000 {
            asm::nop();
        }
        //asm::wfi();
    }
}
