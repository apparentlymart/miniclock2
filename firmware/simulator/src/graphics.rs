use graphics::vector::{Rect, Vector};
use sdl2::pixels::Color;
use sdl2::render::RenderTarget;

pub struct SDLGraphics<T: RenderTarget> {
    canvas: sdl2::render::Canvas<T>,
}

impl<T: RenderTarget> SDLGraphics<T> {
    pub fn new(canvas: sdl2::render::Canvas<T>) -> Self {
        Self { canvas: canvas }
    }
}

impl<T: RenderTarget> graphics::Display for SDLGraphics<T> {
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

    fn fill_rect(&mut self, r: Rect) -> Result<(), Self::Error> {
        let size = r.end - r.start;
        self.canvas.set_draw_color(Self::ON);
        self.canvas.fill_rect(sdl2::rect::Rect::new(r.start.0, r.start.1, size.0 as u32, size.1 as u32))?;
        Ok(())
    }
}
