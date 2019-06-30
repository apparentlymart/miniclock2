#![no_std]

pub mod config;
pub mod gfx;
pub mod interface;
pub mod spi4wire;

#[derive(Debug)]
pub struct SSD1322<I: interface::Interface>(pub(crate) I);

impl<I, CommsError> SSD1322<I>
where
    I: interface::Interface<Error = CommsError>,
{
    pub fn new(ifc: I) -> Self {
        Self(ifc)
    }

    pub fn enable_grayscale_table(&mut self) -> Result<(), Error<CommsError>> {
        self.0.cmd_0(0x00).map_err(Error::comms)
    }

    pub fn set_column_addresses(&mut self, start: u8, end: u8) -> Result<(), Error<CommsError>> {
        if start > 116 || end > 116 {
            return Err(Error::Request);
        }
        self.0.cmd_2(0x15, start, end).map_err(Error::comms)
    }

    pub fn write_gdram(&mut self, data: &[u8]) -> Result<(), Error<CommsError>> {
        self.0.cmd_n(0x5c, data).map_err(Error::comms)
    }

    pub fn write_gdram_iter<It: core::iter::IntoIterator<Item = u8>>(
        &mut self,
        data: It,
    ) -> Result<(), Error<CommsError>> {
        match self.0.cmd_n_iter(0x5c, data).map_err(Error::comms) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    pub fn set_row_addresses(&mut self, start: u8, end: u8) -> Result<(), Error<CommsError>> {
        if start > 127 || end > 127 {
            return Err(Error::Request);
        }
        self.0.cmd_2(0x75, start, end).map_err(Error::comms)
    }

    pub fn set_remap_config(&mut self, cfg: config::Remap) -> Result<(), Error<CommsError>> {
        if cfg.com_split_odd_even && cfg.dual_com_mode {
            return Err(Error::Request); // Invalid combination, per the datasheet
        }
        let (a, b) = cfg.protocol_args();
        self.0.cmd_2(0xa0, a, b).map_err(Error::comms)
    }

    pub fn set_display_start_line(&mut self, start: u8) -> Result<(), Error<CommsError>> {
        if start > 127 {
            return Err(Error::Request);
        }
        self.0.cmd_1(0xa1, start).map_err(Error::comms)
    }

    pub fn set_display_offset(&mut self, offset: u8) -> Result<(), Error<CommsError>> {
        if offset > 127 {
            return Err(Error::Request);
        }
        self.0.cmd_1(0xa2, offset).map_err(Error::comms)
    }

    pub fn set_display_mode(&mut self, mode: config::DisplayMode) -> Result<(), Error<CommsError>> {
        use config::DisplayMode::{AllOff, AllOn, Inverted, Normal};
        self.0
            .cmd_0(match mode {
                AllOff => 0xa4,
                AllOn => 0xa5,
                Normal => 0xa6,
                Inverted => 0xa7,
            })
            .map_err(Error::comms)
    }

    pub fn enable_partial_display(&mut self, start: u8, end: u8) -> Result<(), Error<CommsError>> {
        if start > 127 || end > 127 || end < start {
            return Err(Error::Request);
        }
        self.0.cmd_2(0xa8, start, end).map_err(Error::comms)
    }

    pub fn exit_partial_display(&mut self) -> Result<(), Error<CommsError>> {
        self.0.cmd_0(0xa9).map_err(Error::comms)
    }

    pub fn select_functions(&mut self, cfg: config::Function) -> Result<(), Error<CommsError>> {
        self.0
            .cmd_1(0xab, if cfg.internal_vdd_reg { 0x01 } else { 0x00 })
            .map_err(Error::comms)
    }

    pub fn set_sleep_mode(&mut self, sleep: bool) -> Result<(), Error<CommsError>> {
        self.0
            .cmd_0(if sleep { 0xae } else { 0xaf })
            .map_err(Error::comms)
    }

    pub fn set_phase_lengths(
        &mut self,
        p1_dclks: u8,
        p2_dclks: u8,
    ) -> Result<(), Error<CommsError>> {
        // Phase 1 must be between 5 and 31 and must be an odd number
        if p1_dclks < 5 || p1_dclks > 31 || (p1_dclks % 2) == 0 {
            return Err(Error::Request);
        }
        // Phase 2 must be between 3 and 15
        if p2_dclks < 3 || p2_dclks > 15 {
            return Err(Error::Request);
        }

        let p1_raw = (p1_dclks - 1) / 2;
        let p2_raw = p2_dclks;
        let raw = p1_raw | (p2_raw << 4);

        self.0.cmd_1(0xb1, raw).map_err(Error::comms)
    }

    pub fn set_clock(&mut self, freq: u8, div_by_pow_2: u8) -> Result<(), Error<CommsError>> {
        if freq > 0b1111 {
            return Err(Error::Request);
        }
        if div_by_pow_2 > 0b1010 {
            return Err(Error::Request);
        }

        let raw = (freq << 4) | (div_by_pow_2 << 0);
        self.0.cmd_1(0xb3, raw).map_err(Error::comms)
    }

    pub fn set_display_enhancement_a(
        &mut self,
        cfg: config::DisplayEnhancementA,
    ) -> Result<(), Error<CommsError>> {
        let (a, b) = cfg.protocol_args();
        self.0.cmd_2(0xb4, a, b).map_err(Error::comms)
    }

    pub fn set_display_enhancement_b(
        &mut self,
        cfg: config::DisplayEnhancementB,
    ) -> Result<(), Error<CommsError>> {
        let (a, b) = cfg.protocol_args();
        self.0.cmd_2(0xd1, a, b).map_err(Error::comms)
    }

    pub fn set_gpio_states(
        &mut self,
        gpio0: config::GPIOState,
        gpio1: config::GPIOState,
    ) -> Result<(), Error<CommsError>> {
        let gpio0_raw = gpio0.protocol_arg();
        let gpio1_raw = gpio1.protocol_arg();
        let raw = (gpio0_raw << 0) | (gpio1_raw << 2);
        self.0.cmd_1(0xb5, raw).map_err(Error::comms)
    }

    pub fn set_second_precharge_period(&mut self, dclks: u8) -> Result<(), Error<CommsError>> {
        if dclks > 15 {
            return Err(Error::Request);
        }
        self.0.cmd_1(0xb6, dclks).map_err(Error::comms)
    }

    pub fn set_grayscale_table(&mut self, levels: [u8; 15]) -> Result<(), Error<CommsError>> {
        let mut prev = 0 as u8;
        for (i, v) in levels.iter().enumerate() {
            if *v > 180 {
                return Err(Error::Request);
            }
            if i > 0 && *v <= prev {
                return Err(Error::Request);
            }
            prev = *v
        }
        self.0.cmd_n(0xb8, &levels[..]).map_err(Error::comms)
    }

    pub fn set_default_grayscale_table(&mut self) -> Result<(), Error<CommsError>> {
        self.0.cmd_0(0xb9).map_err(Error::comms)
    }

    pub fn set_precharge_voltage_level(&mut self, code: u8) -> Result<(), Error<CommsError>> {
        if code > 0b11111 {
            return Err(Error::Request);
        }
        self.0.cmd_1(0xbb, code).map_err(Error::comms)
    }

    pub fn set_deselect_voltage_level(&mut self, code: u8) -> Result<(), Error<CommsError>> {
        if code > 0b111 {
            return Err(Error::Request);
        }
        self.0.cmd_1(0xbe, code).map_err(Error::comms)
    }

    pub fn set_contrast_current(&mut self, level: u8) -> Result<(), Error<CommsError>> {
        self.0.cmd_1(0xc1, level).map_err(Error::comms)
    }

    pub fn set_master_contrast_current_control(
        &mut self,
        level: u8,
    ) -> Result<(), Error<CommsError>> {
        if level > 0b1111 {
            return Err(Error::Request);
        }
        self.0.cmd_1(0xc7, level).map_err(Error::comms)
    }

    pub fn set_mux_ratio(&mut self, value: u8) -> Result<(), Error<CommsError>> {
        if value < 16 || value > 128 {
            return Err(Error::Request);
        }
        let raw = value - 1;
        self.0.cmd_1(0xca, raw).map_err(Error::comms)
    }

    pub fn set_command_lock(&mut self, locked: bool) -> Result<(), Error<CommsError>> {
        let raw = if locked { 0b00010110 } else { 0b00010010 };
        self.0.cmd_1(0xfd, raw).map_err(Error::comms)
    }

    pub fn release(self) -> I {
        self.0
    }
}

impl<SPI, CS, DC, SPIErr, CSErr, DCErr> SSD1322<spi4wire::SPI4Wire<SPI, CS, DC>>
where
    SPI: embedded_hal::blocking::spi::Write<u8, Error = SPIErr>,
    CS: embedded_hal::digital::v2::OutputPin<Error = CSErr>,
    DC: embedded_hal::digital::v2::OutputPin<Error = DCErr>,
{
    pub fn new_spi(spi: SPI, cs: CS, dc: DC) -> Self {
        Self::new(spi4wire::SPI4Wire::new(spi, cs, dc))
    }
}

#[derive(Debug)]
pub enum Error<CommsErr> {
    Request,
    Protocol,
    Comms(CommsErr),
}

impl<SPIErr> Error<SPIErr> {
    fn comms(e: SPIErr) -> Self {
        Error::Comms(e)
    }
}
