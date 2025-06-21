use std::ptr;

use cgmath::Matrix;
use gl;

use crate::graphics::RendererError;

use super::glslprogram::GLSLProgram;
use crate::sprite::Sprite;

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

    pub fn setup(&mut self, screen_width: u32, screen_height: u32) -> Result<(), RendererError> {
        let sprite_vertex_shader_source = include_str!("sprite.vert");
        let sprite_fragment_shader_source = include_str!("sprite.frag");

        self.program
            .load_program(sprite_vertex_shader_source, sprite_fragment_shader_source)?;

        unsafe {
            self.vertex_pos_handle = gl::GetAttribLocation(
                self.program.id(),
                "VertexPos\x00".as_bytes().as_ptr().cast(),
            ) as u32;

            self.texcoord_handle = gl::GetAttribLocation(
                self.program.id(),
                "LTexCoord\x00".as_bytes().as_ptr().cast(),
            ) as u32;

            self.modelview_handle =
                gl::GetUniformLocation(self.program.id(), "MV\x00".as_bytes().as_ptr().cast());

            if self.modelview_handle == -1 {
                return Err(RendererError::OtherError(String::from("MV error")));
            }

            gl::UseProgram(self.program.id());

            let proj = cgmath::ortho(
                0.0,
                screen_width as f32,
                0.0,
                screen_height as f32,
                -800.0,
                800.0,
            );

            gl::UniformMatrix4fv(self.program.projection_handle, 1, gl::FALSE, proj.as_ptr());
        }

        Ok(())
    }

    pub fn cleanup(&self) {
        unsafe { gl::UseProgram(0) }
    }

    pub fn set_state(&self) {
        unsafe {
            gl::UseProgram(self.program.id());
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            gl::Enable(gl::BLEND);
        }
    }

    pub fn bind(&self, sprite: &Sprite) {
        sprite.offsets(self.vertex_pos_handle, self.texcoord_handle);
    }

    pub fn set_sprite(&self, sprite: &Sprite) {
        sprite.bind();
    }

    pub fn reset_state() {
        unsafe {
            gl::Disable(gl::BLEND);
            gl::BindVertexArray(gl::ZERO);
        }
    }

    pub fn draw(&self, modelview: &cgmath::Matrix4<f32>) {
        unsafe {
            gl::UniformMatrix4fv(self.modelview_handle, 1, gl::FALSE, modelview.as_ptr());
            gl::DrawArrays(gl::TRIANGLE_STRIP, 0, 4);
        }
    }
}
