use gl;

pub struct Texture{
    tex: Option<gl::types::GLuint>
}

impl Texture{
    pub fn new() -> Texture{
        Texture{
            tex: None
        }
    }

    pub fn create(mut self, file_name: &str) -> Texture{
        let mut tex: gl::types::GLuint;
        unsafe{
            gl::GenTextures(1, &mut tex);
        }

        self
    }
}

impl Drop for Texture{
    fn drop(&mut self) {
        unsafe{
            gl::DeleteTextures(1, &mut self.tex);
        }
    }
}