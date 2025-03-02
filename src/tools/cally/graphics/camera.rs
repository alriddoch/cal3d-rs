use cgmath::Matrix;
use cgmath::SquareMatrix;

pub struct Camera {
    proj: cgmath::Matrix4<f32>,
}

impl Camera {
    pub fn new() -> Self {
        Camera {
            proj: cgmath::Matrix4::<f32>::identity(),
        }
    }

    pub fn setup(&mut self, ratio: f32, depth: f32) {
        self.proj = cgmath::perspective(cgmath::Deg(45.0), ratio, 1.0, depth);
    }

    pub fn projection(&self) -> *const gl::types::GLfloat {
        self.proj.as_ptr()
    }
}
