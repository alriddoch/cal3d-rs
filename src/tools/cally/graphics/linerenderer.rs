use std::ptr;

use cgmath::Matrix;
use gl;

use super::glslprogram::GLSLProgram;

pub struct LineRenderer {
    program: GLSLProgram,

    vao: u32,
    vbo: u32,

    view_handle: i32,
    model_handle: i32,
    vertex_pos_handle: i32,
    color_handle: u32,
}

impl LineRenderer {
    pub fn new() -> Self {
        LineRenderer {
            program: GLSLProgram::new(),
            vao: 0,
            vbo: 0,
            view_handle: 0,
            model_handle: 0,
            vertex_pos_handle: 0,
            color_handle: 0,
        }
    }
}
