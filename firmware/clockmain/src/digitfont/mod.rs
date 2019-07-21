static DIGIT_DATA: &[u8] = include_bytes!("digits.bin");

#[inline(always)]
fn map_ascii_code(ch: char) -> u8 {
    if ch >= '0' && ch <= '9' {
        (ch as u8) - ('0' as u8)
    } else if ch >= 'a' && ch <= 'f' {
        (ch as u8) - ('a' as u8) + 10
    } else {
        0 // we'll just use zero as a placeholder, since we have no other characters
    }
}

const A: u16 = 1 << 0;
const B: u16 = 1 << 1;
const C: u16 = 1 << 2;
const D: u16 = 1 << 3;
const E: u16 = 1 << 4;
const F: u16 = 1 << 5;
const G: u16 = 1 << 6;

const AB: u16 = 1 << 8;
const BC: u16 = 1 << 9;
const CD: u16 = 1 << 10;
const DE: u16 = 1 << 11;
const EF: u16 = 1 << 12;
const FA: u16 = 1 << 13;

#[derive(Clone, Debug)]
pub struct Digit(u16);

impl Digit {
    pub fn get(v: u8) -> Self {
        let nv = v as usize & 0xf;
        Self((DIGIT_DATA[2 * nv + 1] as u16) << 8 | DIGIT_DATA[2 * nv] as u16)
    }

    pub fn get_ascii(ch: char) -> Self {
        Self::get(map_ascii_code(ch))
    }

    pub fn get_raw(raw: u16) -> Self {
        Self(raw)
    }

    #[inline(always)]
    pub fn draw_a(self) -> bool {
        self.0 & A != 0
    }

    #[inline(always)]
    pub fn draw_b(self) -> bool {
        self.0 & B != 0
    }

    #[inline(always)]
    pub fn draw_c(self) -> bool {
        self.0 & C != 0
    }

    #[inline(always)]
    pub fn draw_d(self) -> bool {
        self.0 & D != 0
    }

    #[inline(always)]
    pub fn draw_e(self) -> bool {
        self.0 & E != 0
    }

    #[inline(always)]
    pub fn draw_f(self) -> bool {
        self.0 & F != 0
    }

    #[inline(always)]
    pub fn draw_g(self) -> bool {
        self.0 & G != 0
    }

    #[inline(always)]
    pub fn curve_ab(self) -> bool {
        self.0 & AB != 0
    }

    #[inline(always)]
    pub fn curve_bc(self) -> bool {
        self.0 & BC != 0
    }

    #[inline(always)]
    pub fn curve_cd(self) -> bool {
        self.0 & CD != 0
    }

    #[inline(always)]
    pub fn curve_de(self) -> bool {
        self.0 & DE != 0
    }

    #[inline(always)]
    pub fn curve_ef(self) -> bool {
        self.0 & EF != 0
    }

    #[inline(always)]
    pub fn curve_fa(self) -> bool {
        self.0 & FA != 0
    }

    #[inline(always)]
    pub fn is_one(self) -> bool {
        self.0 == B|C
    }
}

impl Copy for Digit {}
