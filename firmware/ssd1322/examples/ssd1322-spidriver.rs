use serial_embedded_hal::{PortSettings, Serial};
use spidriver::SPIDriver;
use spidriver_hal::SPIDriverHAL;
use ssd1322::SSD1322;

fn main() {
    // This example configures a NHD-3.12-25664UCY2 display module (with
    // integrated SSD1322) and displays a checkerboard pattern on it, accessing
    // the display module via 4-wire SPI via a SPIDriver board on /dev/ttyUSB0.
    //
    // As well as the SPI signals, this example assumes:
    //    SPIDriver Port A is connected to the D/C signal on the driver.
    //    SPIDriver Port B is connected to the reset signal on the driver.

    let port = Serial::new(
        "/dev/ttyUSB0",
        &PortSettings {
            baud_rate: serial_embedded_hal::BaudRate::BaudOther(460800),
            char_size: serial_embedded_hal::CharSize::Bits8,
            parity: serial_embedded_hal::Parity::ParityNone,
            stop_bits: serial_embedded_hal::StopBits::Stop1,
            flow_control: serial_embedded_hal::FlowControl::FlowNone,
        },
    )
    .unwrap();
    let (tx, rx) = port.split();

    let mut sd = SPIDriver::new(tx, rx);

    // Pulse the reset signal to reset the display driver chip before we do
    // anything else.
    sd.set_b(false).unwrap();
    sd.set_b(true).unwrap();

    let sdh = SPIDriverHAL::new(sd);
    let parts = sdh.split();
    let int_spi = ssd1322::spi4wire::SPI4Wire::new(parts.spi, parts.cs, parts.pin_a);
    let int_debug = DebugLogInterface(int_spi);
    let mut driver = SSD1322::new(int_debug);

    // If you only need SPI (no debug wrapper) then this is easier:
    //let mut driver = SSD1322::new_spi(parts.spi, parts.cs, parts.pin_a);

    init(&mut driver).unwrap();

    // We'll allocate a buffer to render our checkerboard pattern into, and then
    // stream it over to the display.
    const WIDTH: usize = 256;
    const HEIGHT: usize = 64;
    const BPP: usize = 4;
    const BUF_SIZE: usize = HEIGHT * WIDTH / (8 / BPP);
    let mut buf: [u8; BUF_SIZE] = [0; BUF_SIZE];
    for y in 0..HEIGHT {
        // each byte represents two pixels
        for x in 0..(WIDTH / 2) {
            buf[(y * WIDTH / 2 + x)] = if y % 2 == 0 { 0xf0 } else { 0x0f };
        }
    }
    driver.write_gdram(&buf[..]).unwrap();
    //driver.set_display_offset(32).unwrap();
    driver.set_display_start_line(0).unwrap();

    let mut disp = ssd1322::gfx::Display::new(driver);
    {
        use graphics::Display;
        //disp.clear().unwrap();
        disp.fill_rect(graphics::vector::Rect::new4(4, 4, 16, 16))
            .unwrap();
    }

    /*loop {
        driver.exit_partial_display().unwrap();
        std::thread::sleep_ms(1000);
        for i in 0..31 {
            driver.enable_partial_display(i, 63 - i).unwrap();
            std::thread::sleep_ms(5);
        }
        driver
            .set_display_mode(ssd1322::config::DisplayMode::AllOff)
            .unwrap();
        std::thread::sleep_ms(1000);
        driver
            .set_display_mode(ssd1322::config::DisplayMode::Normal)
            .unwrap();
        for i in 0..31 {
            driver.enable_partial_display(31 - i, 32 + i).unwrap();
            std::thread::sleep_ms(5);
        }
    }*/
}

fn init<I: ssd1322::interface::Interface>(
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
    drv.set_sleep_mode(false)?; // power display back on

    Ok(())
}

struct DebugLogInterface<I: ssd1322::interface::Interface>(I);

impl<I, Err> ssd1322::interface::Interface for DebugLogInterface<I>
where
    I: ssd1322::interface::Interface<Error = Err>,
{
    type Error = Err;

    fn cmd_0(&mut self, cmd: u8) -> Result<(), Err> {
        println!("command {:#04x}", cmd);
        self.0.cmd_0(cmd)
    }

    fn cmd_1(&mut self, cmd: u8, a: u8) -> Result<(), Err> {
        println!("command {:#04x}: {:#04x}", cmd, a);
        self.0.cmd_1(cmd, a)
    }

    fn cmd_2(&mut self, cmd: u8, a: u8, b: u8) -> Result<(), Err> {
        println!("command {:#04x}: {:#04x} {:#04x}", cmd, a, b);
        self.0.cmd_2(cmd, a, b)
    }

    fn cmd_n(&mut self, cmd: u8, data: &[u8]) -> Result<(), Err> {
        println!("command {:#04x} ({} bytes of data)", cmd, data.len());
        self.0.cmd_n(cmd, data)
    }

    fn cmd_n_iter<It: core::iter::IntoIterator<Item = u8>>(
        &mut self,
        cmd: u8,
        data: It,
    ) -> Result<usize, Self::Error> {
        println!("command {:#04x} (with data iterator)", cmd);
        let ct = self.0.cmd_n_iter(cmd, data)?;
        println!("  - iterator produced {} bytes", ct);
        Ok(ct)
    }
}
