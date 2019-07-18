use graphics::Tile;
use graphics::vector::Vector;
use sdl2::pixels::Color;
use std::collections::HashMap;

pub struct SDLGraphics<'a> {
    canvas: sdl2::render::Canvas<sdl2::video::Window>,
    texture_creator: sdl2::render::TextureCreator<sdl2::video::WindowContext>,
    tile_textures: Box<HashMap<u16, sdl2::render::Texture<'a>>>,
}

impl<'a> SDLGraphics<'a> {
    pub fn new(canvas: sdl2::render::Canvas<sdl2::video::Window>) -> Self {
        let texture_creator = canvas.texture_creator();
        Self {
            canvas: canvas,
            texture_creator: texture_creator,
            tile_textures: Box::new(HashMap::new()),
        }
    }
}

impl<'a> graphics::Display for SDLGraphics<'a> {
    type Error = String;
    type P = Color;
    const OFF: Color = Color{r: 0, g: 0, b: 0, a: 255};
    const ON: Color = Color{r: 255, g: 255, b: 0, a: 255};

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
        let texture = match self.tile_textures.get(&raw_tile) {
            Some(tx) => tx,
            None => {
                let tx = self.texture_creator.create_texture_target(None, 4, 4).unwrap();
                tx.with_lock(None, |buffer: &mut [u8], pitch: usize| {
                    for y in 0..4 {
                        for x in 0..4 {
                            if tile_get_pixel(raw_tile, Vector(x, y)) {
                                let offset = (y as usize)*pitch + (x as usize)*3;
                                buffer[offset] = Self::ON.r;
                                buffer[offset+1] = Self::ON.g;
                                buffer[offset+2] = Self::ON.b;
                            }
                        }
                    }
                });
                self.tile_textures.insert(raw_tile, tx);
                &tx
            },
        };
        
        Ok(())
    }
}

fn tile_get_pixel(raw: u16, p: Vector) -> bool {
    let row = (raw << (p.1 * 4)) & 0xf;
    let px = row << p.0 & 0x1;
    return px != 0;
}
