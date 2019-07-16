static GLYPH_DATA: &[u8] = include_bytes!("blockfont.bin");
static MAP_DATA: &[u8] = include_bytes!("blockfontmap.bin");

fn map_ascii_code(ch: char) -> u8 {
    let v = ch as usize;
    let mut start = 0;
    for i in (0..MAP_DATA.len()).step_by(2) {
        let min = MAP_DATA[i] as usize;
        let max = MAP_DATA[i + 1] as usize;

        if v >= min && v <= max {
            return (v + start - min) as u8;
        }
        start += (max - min) + 1
    }

    // If we fall out here then the character isn't in any of our ranges,
    // so we'll return the placeholder character by default.
    return map_ascii_code(127 as char);
}

#[derive(Debug)]
pub struct Glyph(&'static [u8]);

impl Glyph {
    pub fn get(ch: char) -> Self {
        let idx = map_ascii_code(ch);
        Self::get_idx(idx)
    }

    pub fn get_idx(idx: u8) -> Self {
        let start = (idx as usize) * 15;
        Self(&GLYPH_DATA[start..start + 15])
    }

    pub fn get_tile_idx(self, tx: i32, ty: i32) -> u8 {
        let byte_offset = ((ty * 3) + (tx / 2)) as usize;
        let shift = (tx % 2) as usize;
        (self.0[byte_offset] >> shift) & 0xf
    }
}

impl Copy for Glyph {}

impl Clone for Glyph {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
