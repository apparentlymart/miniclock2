pub struct Display<I: crate::interface::Interface> {
    drv: crate::SSD1322<I>,
    size: graphics::vector::Vector,
    col_offset: u8,
    next_page: i32,
}

impl<I, CommsErr> Display<I>
where
    I: crate::interface::Interface<Error = CommsErr>,
    CommsErr: core::fmt::Debug,
{
    pub fn new(drv: crate::SSD1322<I>, size: graphics::vector::Vector, col_offset: u8) -> Self {
        Self {
            drv: drv,
            size: size,
            col_offset: col_offset,
            next_page: 1,
        }
    }
}

impl<I, CommsErr> graphics::Display for Display<I>
where
    I: crate::interface::Interface<Error = CommsErr>,
    CommsErr: core::fmt::Debug,
{
    type Error = crate::Error<CommsErr>;
    type P = bool;
    const ON: bool = true;
    const OFF: bool = false;

    fn size(&self) -> graphics::vector::Vector {
        self.size
    }

    fn flip(&mut self) -> Result<(), Self::Error> {
        self.drv
            .set_display_start_line((self.size.1 * self.next_page) as u8)?;
        self.next_page = if self.next_page == 0 { 1 } else { 0 };
        Ok(())
    }

    fn clear(&mut self) -> Result<(), Self::Error> {
        let l = raw_buffer_size(self.size());
        let it = core::iter::repeat(0x0).take(l);
        let row_offset = self.next_page * self.size.1;
        let first_row = row_offset as u8;
        let last_row = (row_offset + self.size.1) as u8;

        self.drv.set_column_addresses(
            self.col_offset as u8,
            self.col_offset + (self.size.0 / 4) as u8 - 1,
        )?;
        self.drv.set_row_addresses(first_row, last_row - 1)?;
        self.drv.write_gdram_iter(it)
    }

    fn fill_rect(&mut self, r: graphics::vector::Rect) -> Result<(), Self::Error> {
        if r.start.0 % 4 != 0 || r.end.0 % 4 != 0 {
            // We can only address whole columns, which each include four pixels.
            return Err(crate::Error::<CommsErr>::Request);
        }

        let row_offset = self.next_page * self.size.1;

        let l = raw_buffer_size(r.size());
        let it = core::iter::repeat(0xff).take(l);
        let first_col = self.col_offset + (r.start.0 / 4) as u8;
        let last_col = self.col_offset + (r.end.0 / 4) as u8;
        let first_row = (row_offset + r.start.1) as u8;
        let last_row = (row_offset + r.end.1) as u8;
        self.drv.set_column_addresses(first_col, last_col - 1)?;
        self.drv.set_row_addresses(first_row, last_row - 1)?;
        self.drv.write_gdram_iter(it)
    }
}

fn raw_buffer_size(size: graphics::vector::Vector) -> usize {
    (size.0 as usize / 2) * size.1 as usize // Each byte contains data for two pixels
}
