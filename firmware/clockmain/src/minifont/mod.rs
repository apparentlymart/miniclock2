
static DESCRIPTORS: &[u8] = include_bytes!("descriptors.bin");
static COMMANDS: &[u8] = include_bytes!("commands.bin");

#[inline(always)]
fn map_ascii_code(ch: u8) -> u8 {
    if ch >= 0x41 && ch <= 0x5a {
        ch - 0x41
    } else if ch >= 0x30 && ch <= 0x39 {
        ch - 0x30 + 26
    } else if ch >= 0x20 && ch <= 0x21 {
        ch - 0x20 + 26 + 10
    } else if ch >= 0x7b && ch < 128 {
        ch - 0x7b + 26 + 10 + 2
    } else {
        127 // placeholder character
    }
}

pub fn commands_for_character(ch: u8) -> &'static [u8] {
    let didx = map_ascii_code(ch) as usize;
    let start = (DESCRIPTORS[didx] as usize) << 8 | (DESCRIPTORS[didx+1] as usize);
    let end = (DESCRIPTORS[didx+2] as usize) << 8 | (DESCRIPTORS[didx+3] as usize);
    &COMMANDS[start..end]
}

pub fn decode_command(cmd: u8) -> Command {
    let width = cmd & 0xf;
    let height = cmd >> 4;
    if width == 0 {
        Command::Skip(height)
    } else {
        Command::Rect { w: width, h: height }
    }
}

pub enum Command {
    Rect{w: u8, h: u8},
    Skip(u8),
}
