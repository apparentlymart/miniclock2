#![no_std]

pub mod vector;

use vector::{Rect, Vector};

pub trait Display {
    type Error: core::fmt::Debug;
    type P;
    const OFF: Self::P;
    const ON: Self::P;

    fn size(&self) -> Vector;
    fn flip(&mut self) -> Result<(), Self::Error>;
    fn clear(&mut self) -> Result<(), Self::Error>;
    fn draw_tile<T: Tile>(&mut self, tile: T, pos: Vector) -> Result<(), Self::Error>;
}

pub trait Tile {
    fn raw_pixel_data(&self) -> u16;
}
