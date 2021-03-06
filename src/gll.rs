use gl;
use std;
use result::Result;

// TODO(#31): create an entity for the uniformed location

pub struct UniformLoc<'a> {
    program: &'a Program,
    loc_id: i32
}

impl<'a> UniformLoc<'a> {
    pub fn from_program(program: &'a Program, name: &str) -> UniformLoc<'a> {
        UniformLoc {
            program: program,
            loc_id: unsafe {
                gl::GetUniformLocation(
                    program.id,
                    std::ffi::CString::new(name).unwrap().as_ptr())
            }
        }
    }

    pub fn assign_3fv(&self, v: [f32; 3]) {
        unsafe {
            self.program.use_program();
            gl::Uniform3fv(self.loc_id, 1, v.as_ptr());
        }
    }

    pub fn assign_f32(&self, f: f32) {
        unsafe {
            self.program.use_program();
            gl::Uniform1f(self.loc_id, f);
        }
    }
}

pub struct Program {
    pub id: u32
}

impl Program {
    pub fn from_shaders(shaders: Vec<&Shader>) -> Result<Program> {
        unsafe {
            let id = gl::CreateProgram();

            for shader in shaders.iter() {
                gl::AttachShader(id, shader.id);
            }

            gl::LinkProgram(id);

            let mut params: i32 = -1;

            gl::GetProgramiv(id, gl::LINK_STATUS,
                             &mut params as *mut i32);

            if gl::TRUE as i32 != params {
                let mut max_length: i32 = 0;
                gl::GetProgramiv(id,
                                 gl::INFO_LOG_LENGTH,
                                 &mut max_length as *mut i32);

                let mut error_log: Vec<u8> = vec![0; max_length as usize];

                gl::GetProgramInfoLog(id, max_length,
                                      &mut max_length as *mut i32,
                                      error_log.as_mut_ptr() as *mut i8);
                Err(std::str::from_utf8(&error_log)?.into())
            } else {
                Ok(Program { id: id })
            }
        }
    }

    pub fn use_program(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }
}

pub struct Shader {
    pub id: u32
}

impl Shader {
    pub fn from_str(shader_type: u32, source_code: &str) -> Result<Shader> {
        let c_source_code = std::ffi::CString::new(source_code)?;
        let p = c_source_code.as_ptr() as *const i8;

        unsafe {
            let id = gl::CreateShader(shader_type);
            gl::ShaderSource(id, 1, &p, std::ptr::null());
            gl::CompileShader(id);

            let mut params: i32 = -1;
            gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut params as *mut i32);
            if gl::TRUE as i32 != params {
                let mut max_length: i32 = 0;
                gl::GetShaderiv(id,
                                gl::INFO_LOG_LENGTH,
                                &mut max_length as *mut i32);

                let mut error_log: Vec<u8> = vec![0; max_length as usize];

                gl::GetShaderInfoLog(id,
                                     max_length,
                                     &mut max_length as *mut i32,
                                     error_log.as_mut_ptr() as *mut i8);

                Err(std::str::from_utf8(&error_log)?.into())
            } else {
                Ok(Shader { id: id })
            }
        }
    }
}
