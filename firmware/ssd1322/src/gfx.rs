pub struct Display<I: crate::interface::Interface>(crate::SSD1322<I>);

// The display is wired such that the leftmost pair of pixels are in column
// 28 as far as the display driver is concerned.
const COLUMN_OFFSET: i32 = 28;

impl<I, CommsErr> Display<I>
where
    I: crate::interface::Interface<Error = CommsErr>,
{
    pub fn new(disp: crate::SSD1322<I>) -> Self {
        Self(disp)
    }
}

impl<I, CommsErr> graphics::Display for Display<I>
where
    I: crate::interface::Interface<Error = CommsErr>,
{
    type Error = crate::Error<CommsErr>;
    type P = bool;
    const ON: bool = true;
    const OFF: bool = false;

    fn size(&self) -> graphics::vector::Vector {
        // NOTE: This is the size of the specific NHD-3.12-25664UCY2 display
        // that we're using in the miniclock project, not something fundamental
        // to the SSD1322 display driver itself.
        graphics::vector::Vector(256, 64)
    }

    fn flip(&self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn clear(&mut self) -> Result<(), Self::Error> {
        let l = raw_buffer_size(self.size());
        let it = core::iter::repeat(0x00).take(l);
        self.0.set_column_addresses(28, 91)?;
        self.0.set_row_addresses(0, 63)?;
        self.0.write_gdram_iter(it)
    }

    fn fill_rect(&mut self, r: graphics::vector::Rect) -> Result<(), Self::Error> {
        let l = raw_buffer_size(r.size());
        let it = core::iter::repeat(0xff).take(l);
        let first_col = COLUMN_OFFSET + r.start.0 / 2;
        let last_col = COLUMN_OFFSET + r.end.0 / 2;
        self.0
            .set_column_addresses(first_col as u8, last_col as u8 - 1)?;
        self.0
            .set_row_addresses(r.start.1 as u8, r.end.1 as u8 - 1)?;
        self.0.write_gdram_iter(it)
    }
}

fn raw_buffer_size(size: graphics::vector::Vector) -> usize {
    size.0 as usize * size.1 as usize / 2 // divided by two because there are two pixels in each byte
}
