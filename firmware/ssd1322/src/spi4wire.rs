#[derive(Debug)]
pub struct SPI4Wire<
    SPI: embedded_hal::blocking::spi::Write<u8>,
    CS: embedded_hal::digital::v2::OutputPin,
    DC: embedded_hal::digital::v2::OutputPin,
> {
    spi: SPI,
    cs: CS,
    dc: DC,
}

impl<SPI, CS, DC, SPIErr, CSErr, DCErr> SPI4Wire<SPI, CS, DC>
where
    SPI: embedded_hal::blocking::spi::Write<u8, Error = SPIErr>,
    CS: embedded_hal::digital::v2::OutputPin<Error = CSErr>,
    DC: embedded_hal::digital::v2::OutputPin<Error = DCErr>,
{
    pub fn new(spi: SPI, cs: CS, dc: DC) -> Self {
        Self {
            spi: spi,
            cs: cs,
            dc: dc,
        }
    }

    pub fn release(self) -> (SPI, CS, DC) {
        (self.spi, self.cs, self.dc)
    }

    fn select(&mut self) -> Result<(), Error<SPIErr, CSErr, DCErr>> {
        self.cs.set_low().map_err(Error::cs)
    }

    fn deselect(&mut self) -> Result<(), Error<SPIErr, CSErr, DCErr>> {
        self.cs.set_high().map_err(Error::cs)
    }

    fn command_mode(&mut self) -> Result<(), Error<SPIErr, CSErr, DCErr>> {
        self.dc.set_low().map_err(Error::dc)
    }

    fn data_mode(&mut self) -> Result<(), Error<SPIErr, CSErr, DCErr>> {
        self.dc.set_high().map_err(Error::dc)
    }

    fn write_byte(&mut self, c: u8) -> Result<(), Error<SPIErr, CSErr, DCErr>> {
        let tmp: [u8; 1] = [c];
        self.spi.write(&tmp[..]).map_err(Error::spi)
    }

    fn write_bytes(&mut self, data: &[u8]) -> Result<(), Error<SPIErr, CSErr, DCErr>> {
        self.spi.write(data).map_err(Error::spi)
    }
}

impl<SPI, CS, DC, SPIErr, CSErr, DCErr> crate::interface::Interface for SPI4Wire<SPI, CS, DC>
where
    SPI: embedded_hal::blocking::spi::Write<u8, Error = SPIErr>,
    CS: embedded_hal::digital::v2::OutputPin<Error = CSErr>,
    DC: embedded_hal::digital::v2::OutputPin<Error = DCErr>,
{
    type Error = Error<SPIErr, CSErr, DCErr>;

    fn cmd_0(&mut self, cmd: u8) -> Result<(), Error<SPIErr, CSErr, DCErr>> {
        self.select()?;
        self.command_mode()?;
        self.write_byte(cmd)?;
        self.deselect()
    }

    fn cmd_1(&mut self, cmd: u8, a: u8) -> Result<(), Error<SPIErr, CSErr, DCErr>> {
        self.select()?;
        self.command_mode()?;
        self.write_byte(cmd)?;
        self.data_mode()?;
        self.write_byte(a)?;
        self.deselect()
    }

    fn cmd_2(&mut self, cmd: u8, a: u8, b: u8) -> Result<(), Error<SPIErr, CSErr, DCErr>> {
        self.select()?;
        self.command_mode()?;
        self.write_byte(cmd)?;
        let msg: [u8; 2] = [a, b];
        self.data_mode()?;
        self.write_bytes(&msg[..])?;
        self.deselect()
    }

    fn cmd_n(&mut self, cmd: u8, data: &[u8]) -> Result<(), Error<SPIErr, CSErr, DCErr>> {
        self.select()?;
        self.command_mode()?;
        self.write_byte(cmd)?;
        let mut remain = data;
        self.data_mode()?;
        while remain.len() > 0 {
            let len: usize = if remain.len() > 64 { 64 } else { remain.len() };
            let (this, next) = remain.split_at(len);
            self.write_bytes(this)?;
            remain = next;
        }
        self.deselect()
    }
}

#[derive(Debug)]
pub enum Error<SPIErr, CSErr, DCErr> {
    Request,
    SPI(SPIErr),
    CS(CSErr),
    DC(DCErr),
}

impl<SPIErr, CSErr, DCErr> Error<SPIErr, CSErr, DCErr> {
    fn spi(e: SPIErr) -> Self {
        Error::SPI(e)
    }
    fn cs(e: CSErr) -> Self {
        Error::CS(e)
    }
    fn dc(e: DCErr) -> Self {
        Error::DC(e)
    }
}
