use gl;
use std::ffi::CString;

pub struct GLSLProgram {
    pub program_id: u32,
    pub projection_handle: i32,
    pub vertex_source: String,
    pub fragment_source: String,
}

fn print_shader_log(shader: u32) {
    unsafe {
        let mut log_capacity: gl::types::GLint = 0;
        gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut log_capacity);
        log_capacity += 1;

        let mut log: Vec<u8> = Vec::with_capacity((log_capacity).try_into().unwrap()); // strings.Repeat("\x00", int(logLength+1))
        let mut log_len: gl::types::GLsizei = 0;
        gl::GetShaderInfoLog(
            shader,
            log_capacity,
            &mut log_len,
            log.as_mut_ptr().cast(),
        );
        log.set_len(log_len as usize);

        let error: CString = CString::from_vec_unchecked(log);
        println!("failed to compile \"{}\"", error.to_string_lossy());
    }
}

fn create_shader(typ: u32, shader_source: &str) -> Result<u32, String> {
    let shader = unsafe { gl::CreateShader(typ) };

    unsafe {
        gl::ShaderSource(
            shader,
            1,
            &shader_source.as_bytes().as_ptr().cast(),
            &shader_source.len().try_into().unwrap(),
        );

        gl::CompileShader(shader);

        let mut status: i32 = 0;
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status);
        if status == 0 {
            print_shader_log(shader);
            return Err(String::from("Shader compilation failed"));
        }
    }

    Ok(shader)
}

fn create_program(vertex_shader_source: &str, fragment_shader_source: &str) -> Result<u32, String> {
    let vertex_shader = match create_shader(gl::VERTEX_SHADER, vertex_shader_source) {
        Ok(result) => result,
        Err(error) => return Err(format!("Error: {}", error)), // .into(),
    };

    let fragment_shader = match create_shader(gl::FRAGMENT_SHADER, fragment_shader_source) {
        Ok(result) => result,
        Err(error) => return Err(format!("Error: {}", error)),
    };

    let program_id = unsafe { gl::CreateProgram() };

    unsafe {
        gl::AttachShader(program_id, vertex_shader);
        gl::AttachShader(program_id, fragment_shader);

        gl::LinkProgram(program_id);

        let mut status: i32 = 0;
        gl::GetProgramiv(program_id, gl::LINK_STATUS, &mut status);
        if status == 0 {
            let mut log_length: i32 = 0;
            let mut ret_length: i32 = 0;
            gl::GetProgramiv(program_id, gl::INFO_LOG_LENGTH, &mut log_length);

            let mut v: Vec<u8> = Vec::with_capacity(log_length as usize);
            gl::GetProgramInfoLog(
                program_id,
                log_length,
                &mut ret_length,
                v.as_mut_ptr().cast(),
            );

            return Err(format!(
                "failed to link program: {}",
                String::from_utf8_lossy(&v)
            ));
        }

        gl::DetachShader(program_id, vertex_shader);
        gl::DetachShader(program_id, fragment_shader);

        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(fragment_shader);
    }
    Ok(program_id)
}

fn delete_program(p: u32) {
    unsafe {
	gl::DeleteProgram(p);
    }
}

impl GLSLProgram {
    pub fn new() -> Self {
        GLSLProgram {
            program_id: 0,
            projection_handle: 0,
            vertex_source: String::from(""),
            fragment_source: String::from(""),
        }
    }

    pub fn load_program(
        &mut self,
        vertex_shader_source: &str,
        fragment_shader_source: &str,
    ) -> Result<(), String> {
        self.program_id = match create_program(vertex_shader_source, fragment_shader_source) {
            Ok(result) => result,
            Err(error) => return Err(format!("Error: {}", error)),
        };

        let uniform = CString::new("PM").unwrap();
        self.projection_handle = unsafe {
            gl::GetUniformLocation(
                self.program_id,
                uniform.as_ptr(), /*  "PM\x00".as_bytes().as_ptr().cast()*/
            )
        };
        Ok(())
    }

    pub fn cleanup(&self) {
        delete_program(self.program_id);
    }
}
