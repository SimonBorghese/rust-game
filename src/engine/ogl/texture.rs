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

    pub fn create(mut self) -> Texture{
        let mut tex: gl::types::GLuint = 0;
        unsafe{
            gl::GenTextures(1, &mut tex);
            gl::BindTexture(gl::TEXTURE_2D, tex);

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as _);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as _);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR_MIPMAP_LINEAR as _);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as _);
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }

        self.tex = Some(tex);
        self
    }

    pub fn load(self, width: u32, height: u32, data: *const u8) -> Texture{
        unsafe{
            gl::BindTexture(gl::TEXTURE_2D, self.tex
                .expect("Tried to load texture without creation first!"));
            gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA as gl::types::GLint,
                           width as gl::types::GLsizei, height as gl::types::GLsizei,
                           0, gl::RGBA, gl::UNSIGNED_BYTE, data.cast());
            gl::GenerateMipmap(gl::TEXTURE_2D);

            gl::BindTexture(gl::TEXTURE_2D, 0);
        }

        self
    }

    pub fn bind(&self, unit: u32){
        unsafe{
            gl::ActiveTexture(gl::TEXTURE0 + unit);
            gl::BindTexture(gl::TEXTURE_2D, self.tex
                .expect("Bound texture that doesn't exist!"));
        }
    }
}

impl Drop for Texture{
    fn drop(&mut self) {
        unsafe{
            gl::DeleteTextures(1, &mut self.tex
                .expect("Dropped texture without creation"));
        }
    }
}