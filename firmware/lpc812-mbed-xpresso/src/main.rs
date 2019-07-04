#![no_std]
#![no_main]

extern crate cortex_m_rt;

// We don't have enough flash space for all of the usual panic formatting
// machinery, so we'll just halt on panic.
extern crate panic_halt;

use lpc81x_hal as hal;
use ssd1322::SSD1322;

#[rtfm::app(device = lpc81x_hal)]
const APP: () = {
    static mut LED_PIN: hal::pins::pin::Pin17<hal::pins::mode::DigitalOutput> = ();
    static mut DISPLAY: graphics::scale::ScaleDisplay<
        ssd1322::gfx::Display<
            ssd1322::spi4wire::SPI4Wire<
                lpc81x_hal::spi::SPI0<
                    lpc81x_hal::spi::mode::Host,
                    lpc81x_hal::pins::mode::Assigned<
                        lpc81x_hal::pins::pin::Pin12<lpc81x_hal::pins::mode::Unassigned>,
                    >,
                    lpc81x_hal::pins::mode::Assigned<
                        lpc81x_hal::pins::pin::Pin14<lpc81x_hal::pins::mode::Unassigned>,
                    >,
                    lpc81x_hal::pins::mode::Unassigned,
                    lpc81x_hal::pins::mode::Unassigned,
                >,
                lpc81x_hal::pins::pin::Pin13<lpc81x_hal::pins::mode::DigitalOutput>,
                lpc81x_hal::pins::pin::Pin15<lpc81x_hal::pins::mode::DigitalOutput>,
            >,
        >,
    > = ();

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

        init::LateResources {
            DISPLAY: disp,
            LED_PIN: led_pin,
        }
    }

    #[exception(resources = [LED_PIN, DISPLAY])]
    fn SysTick() {
        static mut EIGHT: bool = true;

        {
            use embedded_hal::digital::v2::ToggleableOutputPin;
            resources.LED_PIN.toggle().unwrap();
        }

        {
            use graphics::vector::Rect;
            use graphics::Display; // Make display trait methods visible on "disp"

            let disp = &mut resources.DISPLAY;

            // Our Display interface uses double-buffering, so we're always
            // drawing to an off-screen buffer here and then flip() instructs
            // the display driver to use that other part of its memory so we
            // can do an atomic transition from one frame to the next.
            disp.clear().unwrap();
            disp.fill_rect(Rect::new4(0, 0, 8, 1)).unwrap();
            disp.fill_rect(Rect::new4(0, 0, 1, 16)).unwrap();
            disp.fill_rect(Rect::new4(7, 0, 8, 16)).unwrap();
            if *EIGHT {
                disp.fill_rect(Rect::new4(0, 7, 8, 8)).unwrap();
            }
            disp.fill_rect(Rect::new4(0, 15, 8, 16)).unwrap();

            // Show the new graphics frame on the display.
            disp.flip().unwrap();

            *EIGHT = !*EIGHT;
        }
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
