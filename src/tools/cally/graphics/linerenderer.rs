use std::ptr;

use cgmath::prelude::*;
use cgmath::Matrix4;
use gl;

use super::camera::Camera;
use super::error::*;
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

pub fn WithOrtho(screen_width: u32, screen_height: u32) -> Matrix4<f32> {
    cgmath::ortho(
        0.0,
        screen_width as f32,
        0.0,
        screen_height as f32,
        -800.0,
        800.0,
    )
}

pub fn WithCamera(camera: &Camera) -> &Matrix4<f32> {
    camera.proj()
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

    pub fn Setup(&mut self, proj: &Matrix4<f32>) -> Result<(), RendererError> {
        let text_vertex_shader_source = include_str!("line.vert");
        let text_fragment_shader_source = include_str!("line.frag");

        self.program
            .load_program(text_vertex_shader_source, text_fragment_shader_source)?;

        unsafe {
            gl::GenVertexArrays(1, &mut self.vao);
            gl::BindVertexArray(self.vao);

            gl::GenBuffers(1, &mut self.vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);

            self.vertex_pos_handle = gl::GetAttribLocation(
                self.program.id(),
                "VertexPos\x00".as_bytes().as_ptr().cast(),
            );

            self.view_handle =
                gl::GetUniformLocation(self.program.id(), "VM\x00".as_bytes().as_ptr().cast());
            if self.view_handle == -1 {
                println!("Error VM");
            }

            self.model_handle =
                gl::GetUniformLocation(self.program.id(), "MM\x00".as_bytes().as_ptr().cast());
            if self.model_handle == -1 {
                println!("Error MM");
            }

            self.color_handle =
                gl::GetUniformLocation(self.program.id(), "color\x00".as_bytes().as_ptr().cast())
                    as u32;

            gl::UseProgram(self.program.id());

            gl::EnableVertexAttribArray(self.vertex_pos_handle as u32);
            gl::VertexAttribPointer(
                self.vertex_pos_handle as u32,
                3,
                gl::FLOAT,
                gl::FALSE,
                0,
                ptr::null(),
            );

            gl::UniformMatrix4fv(self.program.projection_handle, 1, gl::FALSE, proj.as_ptr());

            gl::BindVertexArray(gl::ZERO);

            Ok(())
        }
    }
}
