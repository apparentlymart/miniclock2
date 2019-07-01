#![no_std]
#![no_main]

// We don't have enough flash space for all of the usual panic formatting
// machinery, so we'll just halt on panic.
extern crate panic_halt;

use cortex_m::asm;
use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    asm::nop();

    loop {
        asm::wfi();
    }
}
