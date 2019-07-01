#![no_std]
#![no_main]

extern crate cortex_m_rt;

// We don't have enough flash space for all of the usual panic formatting
// machinery, so we'll just halt on panic.
extern crate panic_halt;

use lpc81x_pac;

#[rtfm::app(device = lpc81x_pac)]
const APP: () = {
    static mut GPIO0: lpc81x_pac::GPIO_PORT = ();

    #[init]
    fn init() -> init::LateResources {
        let cp: rtfm::Peripherals = core;
        let p: lpc81x_pac::Peripherals = device;

        p.GPIO_PORT
            .dir0
            .modify(|r, w| unsafe { w.bits(r.bits() | 1 << 17) });

        let mut syst = cp.SYST;
        syst.set_clock_source(cortex_m::peripheral::syst::SystClkSource::Core);
        syst.set_reload(12_000_000);
        syst.clear_current();
        syst.enable_counter();
        syst.enable_interrupt();

        init::LateResources { GPIO0: p.GPIO_PORT }
    }

    #[exception(resources = [GPIO0])]
    fn SysTick() {
        resources.GPIO0.not0.write(|w| unsafe { w.bits(1 << 17) });
    }
};
