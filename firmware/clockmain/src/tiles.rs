static DATA: &[u8] = include_bytes!("tiles.bin");

#[derive(Clone, Debug)]
pub struct Tile(u16);

impl Tile {
    #[inline(always)]
    pub fn get_by_index(idx: usize) -> Tile {
        let raw = ((DATA[idx * 2 + 1]) as u16) << 8 + (DATA[idx * 2]) as u16;
        Self::get_raw(raw)
    }

    #[inline(always)]
    pub fn get_raw(raw: u16) -> Tile {
        Self(raw)
    }

    pub fn bits(self) -> u16 {
        self.0
    }
}

impl Copy for Tile {}
