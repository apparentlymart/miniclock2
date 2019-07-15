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
    return map_ascii_code(127 as char)
}
