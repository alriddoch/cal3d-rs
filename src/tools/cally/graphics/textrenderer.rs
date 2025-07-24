use std::ptr;

use cgmath::Matrix;
use gl;

use super::glslprogram::GLSLProgram;
use super::{font, font::*};
use crate::graphics::GraphicsError;

pub struct TextRenderer {
    program: GLSLProgram,

    text_texture: u32,

    vao: u32,
    vbo: u32,

    modelview_handle: i32,
    character_handle: i32,
    texcoord_handle: u32,
}

impl TextRenderer {
    pub fn new() -> TextRenderer {
        TextRenderer {
            program: GLSLProgram::new(),
            text_texture: 0,
            vao: 0,
            vbo: 0,
            modelview_handle: 0,
            character_handle: 0,
            texcoord_handle: 0,
        }
    }

    pub fn setup(&mut self, screen_width: u32, screen_height: u32) -> Result<(), GraphicsError> {
        let text_vertex_shader_source = include_str!("text.vert");
        let text_fragment_shader_source = include_str!("text.frag");

        self.program
            .load_program(text_vertex_shader_source, text_fragment_shader_source)?;

        unsafe {
            gl::GenTextures(1, &mut self.text_texture);
            gl::BindTexture(gl::TEXTURE_2D, self.text_texture);
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                font::texture_font_internalFormat,
                font::texture_font_width,
                font::texture_font_height,
                0,
                texture_font_format,
                gl::UNSIGNED_BYTE,
                texture_font_pixels.as_ptr().cast(),
            );

            let glerr = gl::GetError();
            if glerr != gl::NO_ERROR {
                println!("TextRenderer GL Error: {glerr}");
            }

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);

            gl::GenBuffers(1, &mut self.vbo);

            let vertices: [f32; 8] = [
                // These can be computed from the vertexID
                //                   id % 2, id / 2
                //                   0, 0,
                //                   16, 0,
                //                   0, 16,
                //                   16, 16,

                //float cx=(float)(glyph%16)/16.0f;      // X Position Of Current Character
                //float cy=(float)(glyph/16)/16.0f;      // Y Position Of Current Character
                //                     +cx      -cy
                // float ltexcoords[] = {
                0.0,
                1.0 - 0.0625,
                0.0625,
                1.0 - 0.0625,
                0.0,
                1.0,
                0.0625,
                1.0,
            ];

            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                std::mem::size_of_val(&vertices) as isize,
                vertices.as_ptr().cast(),
                gl::STATIC_DRAW,
            );

            let tc = gl::GetAttribLocation(
                self.program.program_id,
                "LTexCoord\x00".as_bytes().as_ptr().cast(),
            );
            if tc == -1 {
                return Err(GraphicsError::OtherError(String::from("Error LTexCorod")));
            }
            self.texcoord_handle = tc as u32;

            self.modelview_handle = gl::GetUniformLocation(
                self.program.program_id,
                "MV\x00".as_bytes().as_ptr().cast(),
            );
            if self.modelview_handle == -1 {
                return Err(GraphicsError::OtherError(String::from("Error MV")));
            }

            self.character_handle = gl::GetUniformLocation(
                self.program.program_id,
                "LCharacter\x00".as_bytes().as_ptr().cast(),
            );
            if self.character_handle == -1 {
                return Err(GraphicsError::OtherError(String::from("Error LCharacter")));
            }

            // gl::BindFragDataLocation(br.program.ProgramID, 0, "outputColor\x00".as_bytes().as_ptr().cast());

            gl::UseProgram(self.program.program_id);

            let proj = cgmath::ortho(
                0.0,
                screen_width as f32,
                0.0,
                screen_height as f32,
                -800.0,
                800.0,
            );

            gl::UniformMatrix4fv(self.program.projection_handle, 1, gl::FALSE, proj.as_ptr());

            gl::GenVertexArrays(1, &mut self.vao);
            gl::BindVertexArray(self.vao);

            gl::EnableVertexAttribArray(self.texcoord_handle);

            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            gl::VertexAttribPointer(
                self.texcoord_handle,
                2,
                gl::FLOAT,
                gl::FALSE,
                0,
                ptr::null(),
            );

            gl::BindVertexArray(gl::ZERO);
        }

        Ok(())
    }

    pub fn cleanup(&self) {
        unsafe {
            gl::UseProgram(0);
        }
        self.program.cleanup();
    }

    pub fn set_state(&self) {
        unsafe {
            gl::UseProgram(self.program.program_id);
            gl::BindVertexArray(self.vao);

            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, self.text_texture);

            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            gl::Enable(gl::BLEND);
        }
    }

    pub fn reset_state(&self) {
        unsafe {
            gl::Disable(gl::BLEND);

            gl::BindVertexArray(gl::ZERO);
        }
    }

    pub fn print(&self, str: &str, modelview: &cgmath::Matrix4<f32>) {
        use std::ops::Mul;

        let mut mv = modelview.clone();
        unsafe {
            for c in str.chars() {
                // TOOD(alriddoch): Check for non-ascii values
                let letter = c as i32 - 32;
                gl::Uniform1i(self.character_handle, letter);
                gl::UniformMatrix4fv(self.modelview_handle, 1, gl::FALSE, mv.as_ptr());
                gl::DrawArrays(gl::TRIANGLE_STRIP, 0, 4);
                mv = mv.mul(cgmath::Matrix4::from_translation(cgmath::Vector3 {
                    x: 10.0,
                    y: 0.0,
                    z: 0.0,
                }));
            }
        }
    }
}
