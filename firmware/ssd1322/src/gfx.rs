use graphics::vector::Vector;
use graphics::Tile;

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
        self.next_page = 1 - self.next_page;
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

    fn draw_tile<T: Tile>(&mut self, tile: T, pos: Vector) -> Result<(), Self::Error> {
        let to_draw = DrawTile::new(tile);

        let phys_pos = Vector(pos.0, pos.1 * 4);
        let offset = Vector(self.col_offset as i32, self.next_page * self.size.1);
        let real_pos = phys_pos + offset;
        let col_addr = real_pos.0 as u8;
        let row_addr = real_pos.1 as u8;
        let raw_bytes = to_draw.raw_gdram_data();
        self.drv.set_column_addresses(col_addr, col_addr)?;
        self.drv.set_row_addresses(row_addr, row_addr + 3)?;
        self.drv.write_gdram(&raw_bytes[..])?;

        Ok(())
    }
}

#[derive(Clone, Debug)]
struct DrawTile([u8; 8]);

impl DrawTile {
    fn new<T: Tile>(tile: T) -> Self {
        let raw = tile.raw_pixel_data();
        let mut ret = Self([0 as u8; 8]);

        // FIXME: This is not the most efficient way to swizzle this from
        // 1bpp to 4bpp, but since the orderings of the pixels and the bits
        // within the pixels doesn't exactly match in the 1bpp and 4bpp forms
        // this is at least _correct_, and we'll consider optimizing it later.
        for y in 0..4 {
            for x in 0..4 {
                let row_mask = 0xf << y * 4;
                let row = (raw & row_mask) >> y * 4;
                let col_mask = 1 << (3 - x);
                if row & col_mask != 0 {
                    ret.set_pixel(Vector(x, y));
                }
            }
        }

        ret
    }

    fn set_pixel(&mut self, v: Vector) {
        let i = ((v.1 * 2) + (v.0 / 2)) as usize;
        let s = (1 - (v.0 % 2)) * 4;
        let m = (0xf << s) as u8;
        self.0[i] |= m;
    }

    fn raw_gdram_data(self) -> [u8; 8] {
        return self.0;
    }
}

impl Copy for DrawTile {}

fn raw_buffer_size(size: graphics::vector::Vector) -> usize {
    (size.0 as usize / 2) * size.1 as usize // Each byte contains data for two pixels
}
