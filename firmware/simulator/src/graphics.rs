use graphics::vector::Vector;
use graphics::Tile;
use sdl2::pixels::Color;
use std::collections::HashMap;
use std::sync::Mutex;

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

pub struct SDLGraphics<'a> {
    canvas: sdl2::render::Canvas<sdl2::video::Window>,
    tile_textures: Box<HashMap<u16, sdl2::render::Texture<'a>>>,
}

impl<'a> SDLGraphics<'a> {
    pub fn new<TILE: Tile>(
        canvas: sdl2::render::Canvas<sdl2::video::Window>,
        tiles: &[TILE],
    ) -> Self {
        let mut tile_textures = HashMap::<u16, sdl2::render::Texture<'a>>::new();
        {
            let texture_creator = canvas.texture_creator();
            for tile in tiles {
                let raw_tile = tile.raw_pixel_data();
                let mut tx = texture_creator.create_texture_target(None, 4, 4).unwrap();
                tx.with_lock(None, |buffer: &mut [u8], pitch: usize| {
                    for y in 0..4 {
                        for x in 0..4 {
                            if tile_get_pixel(raw_tile, Vector(x, y)) {
                                let offset = (y as usize) * pitch + (x as usize) * 3;
                                buffer[offset] = ON.r;
                                buffer[offset + 1] = ON.g;
                                buffer[offset + 2] = ON.b;
                            }
                        }
                    }
                });
                tile_textures.insert(raw_tile, tx);
            }
        }
        Self {
            canvas: canvas,
            tile_textures: Box::new(tile_textures),
        }
    }
}

impl<'a> graphics::Display for SDLGraphics<'a> {
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
        match self.tile_textures.get(&raw_tile) {
            Some(tx) => {}
            None => {}
        }
        Ok(())
    }
}

/*
struct TileCache<'a> {
    tile_textures: Mutex<HashMap<u16, sdl2::render::Texture<'a>>>,
    on_color: Color,
}

impl<'a> TileCache<'a> {
    fn new(on_color: Color) -> Self {
        Self {
            tile_textures: Mutex::new(HashMap::new()),
            on_color: on_color,
        }
    }

    fn get_tile_texture<TILE: Tile>(&'a self, canvas: &sdl2::render::Canvas<sdl2::video::Window>, tile: TILE) -> &'a sdl2::render::Texture<'a> {
        let raw_tile = tile.raw_pixel_data();
        let txs = self.tile_textures.lock().unwrap();

        txs.entry(raw_tile).or_insert_with(|| {
            let texture_creator = canvas.texture_creator();
            let tx = texture_creator.create_texture_target(None, 4, 4).unwrap();
            tx.with_lock(None, |buffer: &mut [u8], pitch: usize| {
                for y in 0..4 {
                    for x in 0..4 {
                        if tile_get_pixel(raw_tile, Vector(x, y)) {
                            let offset = (y as usize)*pitch + (x as usize)*3;
                            buffer[offset] = self.on_color.r;
                            buffer[offset+1] = self.on_color.g;
                            buffer[offset+2] = self.on_color.b;
                        }
                    }
                }
            });
            tx
        })
    }
}
*/

fn tile_get_pixel(raw: u16, p: Vector) -> bool {
    let row = (raw << (p.1 * 4)) & 0xf;
    let px = row << p.0 & 0x1;
    return px != 0;
}
