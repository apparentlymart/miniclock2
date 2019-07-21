use crate::digitfont::Digit;
use crate::tiles::Tile;
use graphics::vector::Vector;
//use crate::blockfont::Glyph;

pub fn draw_big_digit<Display: graphics::Display>(
    num: u8,
    disp: &mut Display,
    top_left: Vector,
) -> Result<(), Display::Error> {
    let digit = Digit::get(num);
    draw_big_digit_raw(digit, disp, top_left)
}

pub fn draw_big_digit_raw<Display: graphics::Display>(
    digit: crate::digitfont::Digit,
    disp: &mut Display,
    top_left: Vector,
) -> Result<(), Display::Error> {
    let ofs = top_left;
    const FILLED: Tile = Tile::FILLED;

    for x in 2..6 {
        if digit.draw_a() {
            disp.draw_tile(FILLED, ofs + Vector(x, 0))?;
            disp.draw_tile(FILLED, ofs + Vector(x, 1))?;
        }
        if digit.draw_d() {
            disp.draw_tile(FILLED, ofs + Vector(x, 14))?;
            disp.draw_tile(FILLED, ofs + Vector(x, 15))?;
        }
        if digit.draw_g() {
            disp.draw_tile(FILLED, ofs + Vector(x, 7))?;
            disp.draw_tile(FILLED, ofs + Vector(x, 8))?;
        }
    }

    for y in 2..7 {
        if digit.draw_b() {
            disp.draw_tile(FILLED, ofs + Vector(6, y))?;
            disp.draw_tile(FILLED, ofs + Vector(7, y))?;
        }
        if digit.draw_f() {
            disp.draw_tile(FILLED, ofs + Vector(0, y))?;
            disp.draw_tile(FILLED, ofs + Vector(1, y))?;
        }
    }

    for y in 9..14 {
        if digit.draw_c() {
            disp.draw_tile(FILLED, ofs + Vector(6, y))?;
            disp.draw_tile(FILLED, ofs + Vector(7, y))?;
        }
        if digit.draw_e() {
            disp.draw_tile(FILLED, ofs + Vector(0, y))?;
            disp.draw_tile(FILLED, ofs + Vector(1, y))?;
        }
    }

    if digit.draw_f() || digit.draw_a() {
        if digit.curve_fa() {
            let corner = Tile::get_by_index(10);
            let dot = Tile::get_by_index(2);
            disp.draw_tile(corner, ofs + Vector(0, 0))?;
            disp.draw_tile(dot, ofs + Vector(2, 2))?;
        } else {
            disp.draw_tile(FILLED, ofs + Vector(0, 0))?;
        }
        disp.draw_tile(FILLED, ofs + Vector(1, 0))?;
        disp.draw_tile(FILLED, ofs + Vector(0, 1))?;
        disp.draw_tile(FILLED, ofs + Vector(1, 1))?;
    }

    if digit.draw_a() || digit.draw_b() {
        if digit.curve_ab() {
            let corner = Tile::get_by_index(12);
            let dot = Tile::get_by_index(1);
            disp.draw_tile(corner, ofs + Vector(7, 0))?;
            disp.draw_tile(dot, ofs + Vector(5, 2))?;
        } else {
            disp.draw_tile(FILLED, ofs + Vector(7, 0))?;
        }
        disp.draw_tile(FILLED, ofs + Vector(6, 0))?;
        disp.draw_tile(FILLED, ofs + Vector(7, 1))?;
        disp.draw_tile(FILLED, ofs + Vector(6, 1))?;
    }

    if digit.draw_c() || digit.draw_d() {
        if digit.curve_cd() {
            let corner = Tile::get_by_index(8);
            let dot = Tile::get_by_index(3);
            disp.draw_tile(corner, ofs + Vector(7, 15))?;
            disp.draw_tile(dot, ofs + Vector(5, 13))?;
        } else {
            disp.draw_tile(FILLED, ofs + Vector(7, 15))?;
        }
        disp.draw_tile(FILLED, ofs + Vector(6, 15))?;
        disp.draw_tile(FILLED, ofs + Vector(7, 14))?;
        disp.draw_tile(FILLED, ofs + Vector(6, 14))?;
    }

    if digit.draw_d() || digit.draw_e() {
        if digit.curve_de() {
            let corner = Tile::get_by_index(5);
            let dot = Tile::get_by_index(6);
            disp.draw_tile(corner, ofs + Vector(0, 15))?;
            disp.draw_tile(dot, ofs + Vector(2, 13))?;
        } else {
            disp.draw_tile(FILLED, ofs + Vector(0, 15))?;
        }
        disp.draw_tile(FILLED, ofs + Vector(1, 15))?;
        disp.draw_tile(FILLED, ofs + Vector(0, 14))?;
        disp.draw_tile(FILLED, ofs + Vector(1, 14))?;
    }

    if digit.draw_b() || digit.draw_c() {
        if digit.curve_bc() {
            let top_corner = Tile::get_by_index(12);
            let bottom_corner = Tile::get_by_index(8);
            if digit.draw_b() && digit.draw_c() {
                disp.draw_tile(bottom_corner, ofs + Vector(7, 7))?;
                disp.draw_tile(top_corner, ofs + Vector(7, 8))?;
            } else if digit.draw_b() {
                disp.draw_tile(FILLED, ofs + Vector(7, 7))?;
                disp.draw_tile(bottom_corner, ofs + Vector(7, 8))?;
            } else {
                disp.draw_tile(top_corner, ofs + Vector(7, 7))?;
                disp.draw_tile(FILLED, ofs + Vector(7, 8))?;
            }
            if digit.draw_b() {
                let dot = Tile::get_by_index(3);
                disp.draw_tile(dot, ofs + Vector(5, 6))?;
            }
            if digit.draw_c() {
                let dot = Tile::get_by_index(1);
                disp.draw_tile(dot, ofs + Vector(5, 9))?;
            }
        } else {
            disp.draw_tile(FILLED, ofs + Vector(7, 7))?;
            disp.draw_tile(FILLED, ofs + Vector(7, 8))?;
        }
        disp.draw_tile(FILLED, ofs + Vector(6, 7))?;
        disp.draw_tile(FILLED, ofs + Vector(6, 8))?;
    }

    if digit.draw_e() || digit.draw_f() {
        if digit.curve_ef() {
            let top_corner = Tile::get_by_index(10);
            let bottom_corner = Tile::get_by_index(5);
            if digit.draw_e() && digit.draw_f() {
                disp.draw_tile(bottom_corner, ofs + Vector(0, 7))?;
                disp.draw_tile(top_corner, ofs + Vector(0, 8))?;
            } else if digit.draw_e() {
                disp.draw_tile(top_corner, ofs + Vector(0, 7))?;
                disp.draw_tile(FILLED, ofs + Vector(0, 8))?;
            } else {
                disp.draw_tile(FILLED, ofs + Vector(0, 7))?;
                disp.draw_tile(bottom_corner, ofs + Vector(0, 8))?;
            }
            if digit.draw_e() {
                let dot = Tile::get_by_index(2);
                disp.draw_tile(dot, ofs + Vector(2, 9))?;
            }
            if digit.draw_f() {
                let dot = Tile::get_by_index(6);
                disp.draw_tile(dot, ofs + Vector(2, 6))?;
            }
        } else {
            disp.draw_tile(FILLED, ofs + Vector(0, 7))?;
            disp.draw_tile(FILLED, ofs + Vector(0, 8))?;
        }
        disp.draw_tile(FILLED, ofs + Vector(1, 7))?;
        disp.draw_tile(FILLED, ofs + Vector(1, 8))?;
    }

    if digit.is_one() {
        // As a special case, we draw a little serif pointy bit at the top
        // of the one, just to make the character box for it feel a little
        // less unbalanced.
        let diag = Tile::get_by_index(10);
        disp.draw_tile(diag, ofs + Vector(5, 0))?;
        disp.draw_tile(diag, ofs + Vector(4, 1))?;
        disp.draw_tile(FILLED, ofs + Vector(5, 1))?;
    }

    Ok(())
}

pub fn draw_colon<Display: graphics::Display>(
    disp: &mut Display,
    top_left: Vector,
) -> Result<(), Display::Error> {
    let tile_top_left = Tile::get_by_index(2).invert();
    let tile_top_right = Tile::get_by_index(1).invert();
    let tile_bottom_left = Tile::get_by_index(6).invert();
    let tile_bottom_right = Tile::get_by_index(3).invert();

    disp.draw_tile(tile_top_left, top_left + Vector(0, 5))?;
    disp.draw_tile(tile_top_left, top_left + Vector(0, 9))?;
    disp.draw_tile(tile_top_right, top_left + Vector(1, 5))?;
    disp.draw_tile(tile_top_right, top_left + Vector(1, 9))?;
    disp.draw_tile(tile_bottom_left, top_left + Vector(0, 6))?;
    disp.draw_tile(tile_bottom_left, top_left + Vector(0, 10))?;
    disp.draw_tile(tile_bottom_right, top_left + Vector(1, 6))?;
    disp.draw_tile(tile_bottom_right, top_left + Vector(1, 10))?;

    Ok(())
}
