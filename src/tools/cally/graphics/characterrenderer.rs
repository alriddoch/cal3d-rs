use cal3d::CalIndex;
use cgmath::Matrix;
use gl;
use std::os::raw::c_void;

use super::WorldGLSLProgram;
use crate::graphics::RendererError;

const bufferVertices: usize = 30000;
const bufferIndeces: usize = 50000;
const vertexBufferSize: usize = bufferVertices * 3 * std::mem::size_of::<f32>(); // vertex
const normalBufferSize: usize = bufferVertices * 3 * std::mem::size_of::<f32>(); // normal
const texcoordBufferSize: usize = bufferVertices * 2 * std::mem::size_of::<f32>(); // texcoord
const arrayBufferSize: usize = vertexBufferSize + normalBufferSize + texcoordBufferSize;
const indexBufferSize: usize = bufferIndeces * 3 * std::mem::size_of::<CalIndex>();

pub struct CharacterRenderer {
    program: WorldGLSLProgram,

    vao: u32,
    vbo: [u32; 2],

    VertexPosHandle: u32,
    NormalHandle: u32,
    TexCoordHandle: u32,

    AmbientHandle: i32,
    DiffuseHandle: i32,
    SpecularHandle: i32,
    ShininessHandle: i32,

    TexHandle: i32,
}

impl CharacterRenderer {
    pub fn new() -> Self {
        Self {
            program: WorldGLSLProgram::new(),
            vao: 0,
            vbo: [0, 0],
            VertexPosHandle: 0,
            NormalHandle: 0,
            TexCoordHandle: 0,
            AmbientHandle: 0,
            DiffuseHandle: 0,
            SpecularHandle: 0,
            ShininessHandle: 0,
            TexHandle: 0,
        }
    }

    pub fn Setup(&mut self, camera: &super::Camera) -> Result<(), RendererError> {
        let vertex_shader_source = include_str!("character.vert");
        let fragment_shader_source = include_str!("character.frag");

        self.program
            .load(vertex_shader_source, fragment_shader_source)?;

        unsafe {
            self.VertexPosHandle = gl::GetAttribLocation(
                self.program.id(),
                "VertexPos\x00".as_bytes().as_ptr().cast(),
            ) as u32;
            self.NormalHandle =
                gl::GetAttribLocation(self.program.id(), "LNormal\x00".as_bytes().as_ptr().cast())
                    as u32;
            self.TexCoordHandle =
                gl::GetAttribLocation(self.program.id(), "TexCoord\x00".as_bytes().as_ptr().cast())
                    as u32;

            self.AmbientHandle = gl::GetUniformLocation(
                self.program.id(),
                "MaterialAmbientColor\x00".as_bytes().as_ptr().cast(),
            );
            self.DiffuseHandle = gl::GetUniformLocation(
                self.program.id(),
                "MaterialDiffuseColor\x00".as_bytes().as_ptr().cast(),
            );
            self.SpecularHandle = gl::GetUniformLocation(
                self.program.id(),
                "MaterialSpecularColor\x00".as_bytes().as_ptr().cast(),
            );
            self.ShininessHandle = gl::GetUniformLocation(
                self.program.id(),
                "MaterialShininess\x00".as_bytes().as_ptr().cast(),
            );
        }

        if self.AmbientHandle == -1 {
            println!("No ambient\n")
        }
        if self.DiffuseHandle == -1 {
            println!("No diff\n")
        }
        if self.SpecularHandle == -1 {
            println!("No spec\n")
        }
        if self.ShininessHandle == -1 {
            println!("No shine\n")
        }

        self.TexHandle = unsafe {
            gl::GetUniformLocation(self.program.id(), "tex\x00".as_bytes().as_ptr().cast())
        };
        if self.TexHandle == -1 {
            println!("No tex\n")
        }

        unsafe {
            gl::GenVertexArrays(1, &mut self.vao);

            gl::BindVertexArray(self.vao);

            gl::EnableVertexAttribArray(self.VertexPosHandle);
            gl::EnableVertexAttribArray(self.NormalHandle);
            gl::EnableVertexAttribArray(self.TexCoordHandle);

            gl::GenBuffers(2, self.vbo.as_mut_ptr());

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.vbo[1]);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                indexBufferSize as isize,
                std::ptr::null(),
                gl::STREAM_DRAW,
            );

            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo[0]);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                arrayBufferSize as isize,
                std::ptr::null(),
                gl::STREAM_DRAW,
            );

            gl::EnableVertexAttribArray(self.VertexPosHandle);
            gl::EnableVertexAttribArray(self.NormalHandle);
            gl::EnableVertexAttribArray(self.TexCoordHandle);

            gl::VertexAttribPointer(
                self.VertexPosHandle,
                3,
                gl::FLOAT,
                gl::FALSE,
                0,
                std::ptr::null(),
            );
            gl::VertexAttribPointer(
                self.NormalHandle,
                3,
                gl::FLOAT,
                gl::FALSE,
                0,
                vertexBufferSize as *const c_void,
            );
            gl::VertexAttribPointer(
                self.TexCoordHandle,
                2,
                gl::FLOAT,
                gl::FALSE,
                0,
                (vertexBufferSize + normalBufferSize) as *const c_void,
            );

            gl::UseProgram(self.program.id());

            gl::UniformMatrix4fv(
                self.program.program.projection_handle,
                1,
                gl::FALSE,
                camera.projection(),
            );
            gl::Uniform1i(self.TexHandle, 0);
        }

        lightingSetup(&self.program);

        Ok(())
    }
}
