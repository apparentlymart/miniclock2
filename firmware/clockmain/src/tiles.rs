static DATA: &[u8] = include_bytes!("tiles.bin");

#[derive(Clone, Debug)]
pub struct Tile(u16);

impl Tile {
    pub const FILLED: Tile = Tile(0xffff);
    pub const EMPTY: Tile = Tile(0x0000);

    #[inline(always)]
    pub fn get_by_index(idx: usize) -> Tile {
        let hi = (DATA[idx * 2 + 1]) as u16;
        let lo = (DATA[idx * 2]) as u16;
        let raw = hi << 8 | lo;
        Self::get_raw(raw)
    }

    pub fn get_all_prerendered() -> [Tile; 16] {
        let mut ret = [Tile(0); 16];
        for i in 0..16 {
            ret[i] = Self::get_by_index(i);
        }
        ret
    }

    #[inline(always)]
    pub fn get_raw(raw: u16) -> Tile {
        Self(raw)
    }

    pub fn bits(self) -> u16 {
        self.0
    }
}

impl graphics::Tile for Tile {
    #[inline(always)]
    fn raw_pixel_data(&self) -> u16 {
        self.0
    }
}

impl Copy for Tile {}
