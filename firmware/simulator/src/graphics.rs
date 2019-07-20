use graphics::vector::Vector;
use graphics::Tile;
use sdl2::pixels::Color;

const OFF: Color = Color {
    r: 0,
    g: 0,
    b: 0,
    a: 255,
};
const ON: Color = Color {
    r: 255,
    g: 255,
    b: 0,
    a: 255,
};

pub struct SDLGraphics {
    canvas: sdl2::render::Canvas<sdl2::video::Window>,
}

impl SDLGraphics {
    pub fn new(
        canvas: sdl2::render::Canvas<sdl2::video::Window>,
    ) -> Self {
        Self {
            canvas: canvas,
        }
    }
}

impl graphics::Display for SDLGraphics {
    type Error = String;
    type P = Color;
    const OFF: Color = OFF;
    const ON: Color = ON;

    fn size(&self) -> Vector {
        let (w, h) = self.canvas.output_size().unwrap();
        return Vector(w as i32, h as i32);
    }

    fn flip(&mut self) -> Result<(), Self::Error> {
        self.canvas.present();
        Ok(())
    }

    fn clear(&mut self) -> Result<(), Self::Error> {
        self.canvas.set_draw_color(Self::OFF);
        self.canvas.clear();
        Ok(())
    }

    fn draw_tile<TILE: Tile>(&mut self, tile: TILE, pos: Vector) -> Result<(), Self::Error> {
        let raw_tile = tile.raw_pixel_data();
        self.canvas.set_draw_color(ON);
        for y in 0..4 {
            for x in 0..4 {
                if tile_get_pixel(raw_tile, Vector(x, y)) {
                    let phys = sdl2::rect::Point::new(x + pos.0, y + pos.1);
                    self.canvas.draw_point(phys)?;
                }
            }
        }
        Ok(())
    }
}

fn tile_get_pixel(raw: u16, p: Vector) -> bool {
    let row_mask = 0xf << p.1 * 4;
    let row = (raw & row_mask) >> p.1 * 4;
    let col_mask = 1 << p.0;
    return row & col_mask != 0;
}
