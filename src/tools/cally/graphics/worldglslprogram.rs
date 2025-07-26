use super::glslprogram::GLSLProgram;

pub struct WorldGLSLProgram {
    pub program: GLSLProgram,

    pub view_handle: i32,
    pub model_handle: i32,

    pub ambient_color_handle: i32,
    pub ambient_power_handle: i32,
    pub sun_handle: i32,
    pub sun_color_handle: i32,
    pub sun_power_handle: i32,
    pub light_handle: i32,
    pub light_color_handle: i32,
    pub light_power_handle: i32,
}

impl WorldGLSLProgram {
    pub fn new() -> Self {
        let program = GLSLProgram::new();

        WorldGLSLProgram {
            program: program,

            view_handle: 0,
            model_handle: 0,

            ambient_color_handle: 0,
            ambient_power_handle: 0,
            sun_handle: 0,
            sun_color_handle: 0,
            sun_power_handle: 0,
            light_handle: 0,
            light_color_handle: 0,
            light_power_handle: 0,
        }
    }

    pub fn id(&self) -> u32 {
        return self.program.program_id;
    }

    pub fn load(
        &mut self,
        vertex_shader_source: &str,
        fragment_shader_source: &str,
    ) -> Result<(), String> {
        self.program
            .load_program(vertex_shader_source, fragment_shader_source)?;

        let id = self.program.program_id;

        unsafe {
            self.view_handle = gl::GetUniformLocation(id, "VM\x00".as_bytes().as_ptr().cast());
            self.model_handle = gl::GetUniformLocation(id, "MM\x00".as_bytes().as_ptr().cast());

            self.ambient_color_handle =
                gl::GetUniformLocation(id, "AmbientColor\x00".as_bytes().as_ptr().cast());
            self.ambient_power_handle =
                gl::GetUniformLocation(id, "AmbientPower\x00".as_bytes().as_ptr().cast());

            self.sun_handle = gl::GetUniformLocation(
                id,
                "SunDirection_worldspace\x00".as_bytes().as_ptr().cast(),
            );
            self.sun_color_handle =
                gl::GetUniformLocation(id, "SunColor\x00".as_bytes().as_ptr().cast());
            self.sun_power_handle =
                gl::GetUniformLocation(id, "SunPower\x00".as_bytes().as_ptr().cast());

            self.light_handle = gl::GetUniformLocation(
                id,
                "LightPosition_worldspace\x00".as_bytes().as_ptr().cast(),
            );
            self.light_color_handle =
                gl::GetUniformLocation(id, "LightColor\x00".as_bytes().as_ptr().cast());
            self.light_power_handle =
                gl::GetUniformLocation(id, "LightPower\x00".as_bytes().as_ptr().cast());
        }
        Ok(())
    }

    pub fn cleanup(&self) {
        self.program.cleanup();
    }
}
