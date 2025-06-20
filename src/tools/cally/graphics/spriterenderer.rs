use std::ptr;

use cgmath::Matrix;
use gl;

use super::glslprogram::GLSLProgram;

use super::{font, font::*};

pub struct SpriteRenderer {
    program: GLSLProgram,

    modelview_handle: i32,
    vertex_pos_handle: u32,
    texcoord_handle: u32,
}

impl SpriteRenderer {
    pub fn new() -> Self {
        SpriteRenderer {
            program: GLSLProgram::new(),
            modelview_handle: 0,
            vertex_pos_handle: 0,
            texcoord_handle: 0,
        }
    }
}
