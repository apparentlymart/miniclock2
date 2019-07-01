#![no_std]
#![no_main]

// We don't have enough flash space for all of the usual panic formatting
// machinery, so we'll just halt on panic.
extern crate panic_halt;

use core::sync::atomic;
use cortex_m::asm;
use cortex_m_rt::{entry, exception};
use lpc81x_pac;

static TICK_PENDING: atomic::AtomicBool = atomic::AtomicBool::new(false);

#[entry]
fn main() -> ! {
    let p = lpc81x_pac::Peripherals::take().unwrap();
    let cp = lpc81x_pac::CorePeripherals::take().unwrap();

    p.GPIO_PORT
        .dir0
        .modify(|r, w| unsafe { w.bits(r.bits() | 1 << 17) });

    //let mut nvic = cp.NVIC;
    //let mut syscon = cp.SCB;

    let mut syst = cp.SYST;
    syst.set_clock_source(cortex_m::peripheral::syst::SystClkSource::Core);
    syst.set_reload(12_000_000);
    syst.clear_current();
    syst.enable_counter();
    syst.enable_interrupt();

    unsafe {
        cortex_m::interrupt::enable();
    }

    loop {
        if TICK_PENDING.load(atomic::Ordering::SeqCst) {
            p.GPIO_PORT.not0.write(|w| unsafe { w.bits(1 << 17) });
            TICK_PENDING.store(false, atomic::Ordering::SeqCst);
        }
        /*for _ in 0..1000 {
            asm::nop();
        }*/
        asm::wfi();
    }
}

#[exception]
fn SysTick() {
    TICK_PENDING.store(true, atomic::Ordering::SeqCst);
}
