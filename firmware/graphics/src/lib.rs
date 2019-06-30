#![no_std]

pub mod vector;
use vector::{Rect, Vector};

pub trait Display {
    type Error;
    type P;
    const OFF: Self::P;
    const ON: Self::P;

    fn size(&self) -> Vector;
    fn flip(&self) -> Result<(), Self::Error>;
    fn clear(&mut self) -> Result<(), Self::Error>;
    fn fill_rect(&mut self, r: Rect) -> Result<(), Self::Error>;
}
