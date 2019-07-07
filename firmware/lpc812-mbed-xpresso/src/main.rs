#![no_std]
#![no_main]

extern crate cortex_m_rt;

use lpc81x_hal as hal;
use ssd1322::SSD1322;

#[rtfm::app(device = lpc81x_hal)]
const APP: () = {
    static mut EVENTS: clockmain::Events = ();
    //static mut TICKINT: bool = ();
    static mut APP: clockmain::App<
        ds3231::DS3231<
            lpc81x_hal::i2c::I2C<
                lpc81x_hal::pins::mode::Assigned<
                    lpc81x_hal::pins::pin::Pin11<
                        lpc81x_hal::pins::mode::Unassigned,
                    >,
                >,
                lpc81x_hal::pins::mode::Assigned<
                    lpc81x_hal::pins::pin::Pin10<
                        lpc81x_hal::pins::mode::Unassigned,
                    >,
                >,
                lpc81x_hal::i2c::mode::Host<
                    lpc81x_hal::i2c::mode::Active,
                >,
                lpc81x_hal::i2c::mode::Device<
                    lpc81x_hal::i2c::mode::Inactive,
                >,
                lpc81x_hal::i2c::mode::Monitor<
                    lpc81x_hal::i2c::mode::Inactive,
                >,
            >
        >,
        graphics::scale::ScaleDisplay<
            ssd1322::gfx::Display<
                ssd1322::spi4wire::SPI4Wire<
                    lpc81x_hal::spi::SPI0<
                        lpc81x_hal::spi::mode::Host,
                        lpc81x_hal::pins::mode::Assigned<
                            lpc81x_hal::pins::pin::Pin12<
                                lpc81x_hal::pins::mode::Unassigned,
                            >,
                        >,
                        lpc81x_hal::pins::mode::Assigned<
                            lpc81x_hal::pins::pin::Pin14<
                                lpc81x_hal::pins::mode::Unassigned,
                            >,
                        >,
                        lpc81x_hal::pins::mode::Unassigned,
                        lpc81x_hal::pins::mode::Unassigned,
                    >,
                    lpc81x_hal::pins::pin::Pin13<
                        lpc81x_hal::pins::mode::DigitalOutput,
                    >,
                    lpc81x_hal::pins::pin::Pin15<
                        lpc81x_hal::pins::mode::DigitalOutput,
                    >,
                >,
            >,
        >,
    > = ();

    #[init]
    fn init() -> init::LateResources {
        let cp: rtfm::Peripherals = core;
        let p: hal::Peripherals = device;

        let mut syst = cp.SYST;
        syst.set_clock_source(cortex_m::peripheral::syst::SystClkSource::Core);
        syst.set_reload(6_000_000);
        syst.clear_current();
        syst.enable_counter();
        syst.enable_interrupt();

        let spi = p
            .spi0
            .activate_as_host(
                p.pins.gpio12,
                hal::spi::cfg::Config {
                    sclk_mode: embedded_hal::spi::MODE_0,
                    bit_order: hal::spi::cfg::BitOrder::MSBFirst,
                },
            )
            .with_mosi(p.pins.gpio14);
        let cs = p.pins.gpio13.to_digital_output(true);
        let dc = p.pins.gpio15.to_digital_output(false);
        //let mut rst = p.pins.gpio16.to_digital_output(true);

        let mut disp_drv = SSD1322::new_spi(spi, cs, dc);
        init_oled(&mut disp_drv).unwrap();

        // The ssd1322 display driver has column addresses that address four pixels
        // each, and so without buffering the display in local memory we're
        // effectively limited to updating only multiples of four pixels in the
        // horizontal axis. To simplify things for now, we'll just scale everything
        // by 4 and produce chunky 4x4 "pixels".
        let disp = graphics::scale::ScaleDisplay::new(
            ssd1322::gfx::Display::new(disp_drv, graphics::vector::Vector(256, 64), 28),
            4,
        );

        // We'll get clock information from a connected DS3231 over I2C.
        let i2c = p.i2c.activate(p.pins.gpio11, p.pins.gpio10).enable_host_mode();
        let rtc = ds3231::DS3231::new(i2c);

        let app = clockmain::App::new(rtc, disp);

        // TODO: SQW pin interrupt from the RTC

        init::LateResources {
            APP: app,
            EVENTS: clockmain::Events::default(),
        }
    }

    #[idle(resources = [EVENTS, APP])]
    fn idle() -> ! {
        let app = &mut resources.APP;
        loop {
            // We'll temporarily block all of the exceptions/interrupts that
            // might update events while we deal with our update step.
            resources.EVENTS.lock(|events| {
                if events.has_pending() {
                    app.update(events);
                }
                events.reset();
            });
            app.redraw();

            rtfm::export::wfi();
        }
    }

    #[exception(resources = [EVENTS])]
    fn SysTick() {
        resources.EVENTS.tick = true;
    }
};

fn init_oled<I: ssd1322::interface::Interface>(
    drv: &mut SSD1322<I>,
) -> Result<(), ssd1322::Error<I::Error>> {
    // These settings are for the NHD-3.12-25664UCY2 display module, and are
    // derived from its datasheet. Other display modules may need different
    // settings.
    drv.set_command_lock(false)?;
    drv.set_sleep_mode(true)?; // just during init; we'll enable it again later
    drv.set_column_addresses(28, 91)?;
    drv.set_row_addresses(0, 63)?;
    drv.set_clock(9, 1)?;
    drv.set_mux_ratio(64)?;
    drv.set_display_offset(0)?;
    drv.set_display_start_line(0)?;
    drv.set_remap_config(ssd1322::config::Remap {
        address_increment: ssd1322::config::WriteDirection::Horizontal,
        column_addr_remap: false,
        nibble_remap: true,
        scan_direction: ssd1322::config::ScanDirection::Backward,
        com_split_odd_even: false,
        dual_com_mode: true,
    })?;
    drv.set_gpio_states(
        ssd1322::config::GPIOState::HiZ(false),
        ssd1322::config::GPIOState::HiZ(false),
    )?;
    drv.select_functions(ssd1322::config::Function {
        internal_vdd_reg: true,
    })?;
    drv.set_display_enhancement_a(ssd1322::config::DisplayEnhancementA {
        vsl: ssd1322::config::VSL::External,
        low_gs_quality: ssd1322::config::LowGSQuality::Enhanced,
    })?;
    drv.set_contrast_current(0x9f)?;
    drv.set_master_contrast_current_control(0x0f)?;
    drv.set_default_grayscale_table()?;
    drv.set_phase_lengths(5, 14)?;
    drv.set_display_enhancement_b(ssd1322::config::DisplayEnhancementB::Normal)?;
    drv.set_precharge_voltage_level(0x1d)?;
    drv.set_second_precharge_period(8)?;
    drv.set_deselect_voltage_level(0x07)?;
    drv.set_display_mode(ssd1322::config::DisplayMode::Normal)?;

    // Clear out the first page of GDRAM so we won't show garbage while
    // we're waiting for the first real frame to render.
    drv.write_gdram_iter(core::iter::repeat(0x0).take(8192))?;

    drv.set_sleep_mode(false)?; // power display back on

    Ok(())
}

#[inline(never)]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    // Error LED (red LED) is on pin 7.
    const ERR_LED_MASK: u32 = 1 << 7;
    let gpio = lpc81x_pac::GPIO_PORT::ptr();
    loop {
        unsafe { 
            (*gpio).clr0.write(|w| w.bits(ERR_LED_MASK));
            (*gpio)
                .dir0
                .modify(|r, w| w.bits(r.bits() | ERR_LED_MASK));
        }

        // If we panicked before we set up interrupts then the LED will
        // just stay on here, which is fine... but if we got far enough to
        // have the tick interrupt set up then it will blink.
        cortex_m::asm::wfi();

        unsafe { 
            (*gpio).set0.write(|w| w.bits(ERR_LED_MASK));
        }
        cortex_m::asm::wfi();

        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    }
}
