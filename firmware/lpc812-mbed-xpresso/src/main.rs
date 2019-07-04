#![no_std]
#![no_main]

extern crate cortex_m_rt;

// We don't have enough flash space for all of the usual panic formatting
// machinery, so we'll just halt on panic.
extern crate panic_halt;

use lpc81x_hal as hal;

#[rtfm::app(device = lpc81x_hal)]
const APP: () = {
    static mut LED_PIN: hal::pins::pin::Pin17<hal::pins::mode::DigitalOutput> = ();

    #[init]
    fn init() -> init::LateResources {
        let cp: rtfm::Peripherals = core;
        let p: hal::Peripherals = device;

        let led_pin = p.pins.gpio17.to_digital_output(true);

        let mut syst = cp.SYST;
        syst.set_clock_source(cortex_m::peripheral::syst::SystClkSource::Core);
        syst.set_reload(12_000_000);
        syst.clear_current();
        syst.enable_counter();
        syst.enable_interrupt();

        init::LateResources { LED_PIN: led_pin }
    }

    #[exception(resources = [LED_PIN])]
    fn SysTick() {
        use embedded_hal::digital::v2::ToggleableOutputPin;
        resources.LED_PIN.toggle().unwrap();
    }
};
