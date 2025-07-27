use cal3d::CalIndex;
use cgmath::Matrix;
use cgmath::Matrix4;
use cgmath::SquareMatrix;
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

    fn SetState(&self, view: &Matrix4<f32>) -> Result<(), RendererError> {
        unsafe {
            gl::UseProgram(self.program.id());
            gl::BindVertexArray(self.vao);

            gl::UniformMatrix4fv(self.program.view_handle, 1, gl::FALSE, view.as_ptr());
        }

        return Ok(());
    }

    fn ResetState() {
        unsafe { gl::BindVertexArray(gl::ZERO) }
    }

    fn setMaterials(&self, ambient: [f32; 4], diffuse: [f32; 4], specular: [f32; 4], shine: f32) {
        unsafe {
            gl::Uniform4fv(self.AmbientHandle, 1, &ambient[0]);
            gl::Uniform4fv(self.DiffuseHandle, 1, &diffuse[0]);
            gl::Uniform4fv(self.SpecularHandle, 1, &specular[0]);
            gl::Uniform1f(self.ShininessHandle, shine);
        }
    }

    fn mapArrayBuffer(&self) -> Result<*mut c_void, RendererError> {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo[0]);
            let u = gl::MapBuffer(gl::ARRAY_BUFFER, gl::READ_WRITE);
            if u == std::ptr::null_mut() {
                return Err(RendererError::OtherError(String::from(
                    "error mapping array buffer",
                )));
            }
            Ok(u)
        }
    }

    fn unmapArrayBuffer(&self) {
        unsafe {
            gl::UnmapBuffer(gl::ARRAY_BUFFER);
        }
    }

    fn mapElementBuffer(&self) -> Result<*mut c_void, RendererError> {
        unsafe {
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.vbo[1]);
            let u = gl::MapBuffer(gl::ELEMENT_ARRAY_BUFFER, gl::READ_WRITE);
            if u == std::ptr::null_mut() {
                return Err(RendererError::OtherError(String::from(
                    "error mapping element buffer",
                )));
            }
            Ok(u)
        }
    }

    fn unmapElementBuffer(&self) {
        unsafe {
            gl::UnmapBuffer(gl::ELEMENT_ARRAY_BUFFER);
        }
    }

    fn Draw(&self, faces: i32, texture: u32) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, texture);

            // gl::DrawElements(GL_TRIANGLES, faceCount*3, GL_UNSIGNED_INT, &meshFaces[0][0])
            let model = Matrix4::<f32>::identity();
            gl::UniformMatrix4fv(self.program.model_handle, 1, gl::FALSE, model.as_ptr());
            gl::DrawElements(gl::TRIANGLES, faces * 3, gl::UNSIGNED_INT, std::ptr::null());
        }
    }
}

fn lightingSetup(program: &WorldGLSLProgram) {
    unsafe {
        let ambientColor: [f32; 3] = [0.9, 0.9, 1.0];
        gl::Uniform3fv(program.ambient_color_handle, 1, &ambientColor[0]);

        gl::Uniform1f(program.ambient_power_handle, 0.35);

        let sun: [f32; 3] = [0., 0., 1.];
        gl::Uniform3fv(program.sun_handle, 1, &sun[0]);

        let sunColor: [f32; 3] = [1.0, 0.998, 0.975];
        gl::Uniform3fv(program.sun_color_handle, 1, &sunColor[0]);

        gl::Uniform1f(program.sun_power_handle, 0.7);

        let light: [f32; 3] = [80., 80., 80.];
        gl::Uniform3fv(program.light_handle, 1, &light[0]);

        let lightColor: [f32; 3] = [1.0, 0.0, 0.5];
        gl::Uniform3fv(program.light_color_handle, 1, &lightColor[0]);

        gl::Uniform1f(program.light_power_handle, 500.0);
    }
}
