use graphics::vector::{Rect, Vector};

pub fn draw_big_digit<Display: graphics::Display>(num :u8, disp: &mut Display, top_left: Vector) -> Result<(), Display::Error> {
    let d = num & 0xf;
    let st = top_left;

    //       A
    //     -----
    //    |     |
    //  F |     | B
    //    |  G  |
    //     -----
    //    |     |
    //  E |     | C
    //    |     |
    //     -----
    //       D

    // A
    if d == 0 || d == 2 || d == 3 || d == 5 || d == 6 || d == 6 || d == 7 || d == 8 || d == 9 {
        let mut start = 2;
        let mut end = 6;
        if d == 2 || d == 3 || d == 5 || d == 7 {
            start = 0;
        }
        if d == 5 || d == 6 || d == 7 {
            end = 8;
        }

        disp.fill_rect(Rect::new(st+Vector(start, 0), st+Vector(end, 2)))?;
    }

    // B
    if d == 0 || d == 1 || d == 2 || d == 3 || d == 4 || d == 7 || d == 8 || d == 9 {
        let mut start = 2;
        let mut end = 7;
        if d == 1 || d == 4 || d == 7 {
            start = 0;
        }
        if d == 0 || d == 2 || d == 3 || d == 8 || d == 9 {
            start = 1;
        }
        if d == 2 {
            end = 8;
        }
        if d == 0 || d == 1 || d == 4 || d == 7 {
            end = 9;
        }

        disp.fill_rect(Rect::new(st+Vector(6, start), st+Vector(8, end)))?;
    }

    // C
    if d == 0 || d == 1 || d == 3 || d == 4 || d == 5 || d == 6 || d == 7 || d == 8 || d == 9 {
        let mut start = 9;
        let mut end = 14;
        if d == 5 || d == 6 {
            start = 8;
        }
        if d == 0 || d == 3 || d == 5 || d == 6 || d == 8 || d == 9 || d == 0 {
            end = 15;
        }
        if d == 1|| d == 4 || d == 7 {
            end = 16;
        }
        disp.fill_rect(Rect::new(st+Vector(6, start), st+Vector(8, end)))?;
    }

    // D
    if d == 0 || d == 2 || d == 3 || d == 5 || d == 6 || d == 8 || d == 9 {
        let mut start = 2;
        let mut end = 6;
        if d == 2 || d == 3 || d == 5 || d == 9 {
            start = 0;
        }
        if d == 2 {
            end = 8;
        }

        disp.fill_rect(Rect::new(st+Vector(start, 14), st+Vector(end, 16)))?;
    }

    // E
    if d == 0 || d == 2 || d == 6 || d == 8 {
        let mut start = 9;
        let mut end = 14;
        if d == 2 {
            start = 8;
        }
        if d == 0 || d == 6 || d == 8 {
            end = 15;
        }

        disp.fill_rect(Rect::new(st+Vector(0, start), st+Vector(2, end)))?;
    }

    // F
    if d == 0 || d == 4 || d == 5 || d == 6 || d == 8 || d == 9 {
        let mut start = 2;
        let mut end = 7;
        if d == 4 {
            start = 0;
        }
        if d == 0 || d == 6 || d == 8 || d == 9 {
            start = 1;
        }
        if d == 4 || d == 5 || d == 9 {
            end = 8;
        }
        if d == 0 || d == 6 {
            end = 9;
        }
        disp.fill_rect(Rect::new(st+Vector(0, start), st+Vector(2, end)))?;
    }

    // G
    if d == 2 || d == 3 || d == 4 || d == 5 || d == 6 || d == 8 || d == 9 {
        let mut start = 1;
        let mut end = 7;
        if d == 9 || d == 4 {
            end = 8
        }
        if d == 6 {
            start = 0
        }
        if d == 3 {
            start = 2
        }
        
        disp.fill_rect(Rect::new(st+Vector(start, 7), st+Vector(end, 9)))?;
    }

    Ok(())
}

