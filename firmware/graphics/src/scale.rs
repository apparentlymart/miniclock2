use crate::vector::{Rect, Vector};

pub struct ScaleDisplay<D: crate::Display> {
    disp: D,
    scale: i32,
}

impl<D> ScaleDisplay<D>
where
    D: crate::Display,
{
    pub fn new(disp: D, scale: i32) -> Self {
        Self {
            disp: disp,
            scale: scale,
        }
    }
}

impl<D, Err> crate::Display for ScaleDisplay<D>
where
    D: crate::Display<Error = Err>,
{
    type Error = Err;
    type P = D::P;
    const OFF: D::P = D::OFF;
    const ON: D::P = D::ON;

    fn size(&self) -> Vector {
        self.disp.size() / self.scale
    }

    fn flip(&mut self) -> Result<(), Self::Error> {
        self.disp.flip()
    }

    fn clear(&mut self) -> Result<(), Self::Error> {
        self.disp.clear()
    }

    fn fill_rect(&mut self, r: Rect) -> Result<(), Self::Error> {
        self.disp
            .fill_rect(Rect::new(r.start * self.scale, r.end * self.scale))
    }
}
