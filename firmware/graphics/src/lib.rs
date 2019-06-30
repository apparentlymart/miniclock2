#![no_std]

pub mod vector;
use vector::{Rect, Vector};

pub trait Display {
    type Error;
    type P;
    const OFF: Self::P;
    const ON: Self::P;

    fn size(&self) -> Vector;
    fn window(&self, bounds: Rect);
    fn flip(&self) -> Result<(), Self::Error>;
}

pub trait Window<D: Display> {
    type Buffer: Buffer<D::P>;

    fn size(&self) -> Vector;
    fn fill(&self, p: D::P) -> Result<(), D::Error>;
    fn draw<F: FnOnce(&mut Self::Buffer)>(&self, f: F);
    fn sub_window(&self, bounds: Rect);
}

pub trait Buffer<P> {
    fn size(&self) -> Vector;
    fn set(&mut self, x: i32, y: i32, p: P);
    fn get(&mut self, x: i32, y: i32) -> P;
}
