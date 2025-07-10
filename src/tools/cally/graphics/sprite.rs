use std::os::raw::c_void;
use std::path::PathBuf;

pub struct Sprite {
    vao: u32,
    vbo: u32,

    w: u32,
    h: u32,

    spriteTexture: u32,
}

pub enum SpriteError {
    OtherError(String),
}

impl Sprite {
    pub fn new() -> Self {
        Sprite {
            vao: 0,
            vbo: 0,
            w: 0,
            h: 0,
            spriteTexture: 0,
        }
    }

    pub fn WithSpriteFile(&mut self, filename: &PathBuf) -> &mut Sprite {
        println!("Loading {filename:?}");

        let sprite = super::get_sprite(filename);
        if sprite.is_err() {
            println!("Failed to load sprite: {filename:?}");
        }
        (self.spriteTexture, self.w, self.h) = sprite.unwrap();

        let glerr = unsafe { gl::GetError() };

        if glerr != gl::NO_ERROR {
            println!("SpriteRenderer GL Error: {glerr}");
        }

        self
    }

    pub fn Setup(&mut self) -> Result<(), SpriteError> {
        unsafe {
            gl::GenVertexArrays(1, &mut self.vao);
            gl::BindVertexArray(self.vao);

            gl::GenBuffers(1, &mut self.vbo);

            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);

            gl::BindVertexArray(gl::ZERO);

            let glerr = gl::GetError();

            if glerr != gl::NO_ERROR {
                println!("SpriteRenderer GL Error: {glerr}");
            }
        }
        Ok(())
    }

    pub fn offsets(&self, vertexHandle: u32, texcoordHandle: u32) {
        unsafe {
            gl::BindVertexArray(self.vao);

            gl::EnableVertexAttribArray(vertexHandle);
            gl::EnableVertexAttribArray(texcoordHandle);

            gl::VertexAttribPointer(vertexHandle, 2, gl::FLOAT, gl::FALSE, 0, std::ptr::null());
            gl::VertexAttribPointer(
                texcoordHandle,
                2,
                gl::FLOAT,
                gl::FALSE,
                0,
                (4 * 2 * std::mem::size_of::<f32>()) as *const c_void,
            );

            gl::BindVertexArray(gl::ZERO)
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.vao);

            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            let vertices: [f32; 16] = [
                0.0,
                0.0,
                self.w as f32,
                0.0,
                0.0,
                self.h as f32,
                self.w as f32,
                self.h as f32,
                0.0,
                1.0,
                1.0,
                1.0,
                0.0,
                0.0,
                1.0,
                0.0,
            ];
            gl::BufferData(
                gl::ARRAY_BUFFER,
                std::mem::size_of_val(&vertices) as isize, //len(vertices)*int(unsafe.Sizeof(vertices[0])),
                vertices.as_ptr().cast(),
                gl::STREAM_DRAW,
            );

            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, self.spriteTexture);
        }
    }
}
