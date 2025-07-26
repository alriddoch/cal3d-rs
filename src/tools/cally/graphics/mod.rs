mod camera;
mod characterrenderer;
mod error;
mod font;
mod glslprogram;
mod image;
mod linerenderer;
mod screen;
mod sprite;
mod spriterenderer;
mod textrenderer;
mod worldglslprogram;

pub use camera::*;
pub use characterrenderer::*;
pub use error::*;
pub use glslprogram::*;
pub use image::*;
pub use linerenderer::*;
pub use screen::*;
pub use sprite::{Sprite, SpriteError};
pub use spriterenderer::*;
pub use textrenderer::*;
pub use worldglslprogram::*;

#[derive(Debug)]
pub enum GraphicsError {
    OtherError(String),
}

impl From<String> for GraphicsError {
    fn from(error: String) -> Self {
        GraphicsError::OtherError(error)
    }
}
