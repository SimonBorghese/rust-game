use gl;
use gl::types::*;

pub struct Vao{
    pub buf: Option<GLuint>
}

pub struct VertexPointer{
    location: u32,
    size: u32,
    type_value: gl::types::GLenum,
    normalized: bool,
    raw_size: usize,
    pointer: usize
}

impl VertexPointer{
    pub fn new() -> VertexPointer{
        VertexPointer{
            location: 0,
            size: 0,
            type_value: 0,
            normalized: false,
            raw_size: 0,
            pointer: 0,
        }
    }

    pub fn location(&mut self, loc: u32) -> &mut VertexPointer{
        self.location = loc;

        self
    }

    pub fn num_values(&mut self, loc: u32) -> &mut VertexPointer{
        self.size = loc;

        self
    }

    pub fn type_value(&mut self, loc: gl::types::GLenum) -> &mut VertexPointer{
        self.type_value = loc;

        self
    }

    pub fn normalized(&mut self, loc: bool) -> &mut VertexPointer{
        self.normalized = loc;

        self
    }

    pub fn raw_size(&mut self, loc: usize) -> &mut VertexPointer{
        self.raw_size = loc;

        self
    }

    pub fn pointer(&mut self, loc: usize) -> &mut VertexPointer{
        self.pointer = loc;

        self
    }
}

impl Vao{
    pub fn new() -> Vao{

        let mut buf = 0;
        unsafe{
            gl::GenVertexArrays(1, &mut buf);
        }

        assert_ne!(buf, 0);
        Vao{
            buf: Some(buf)
        }
    }

    pub fn bind(&self){
        unsafe{
            gl::BindVertexArray(self.buf.expect("Bound non-existent VAO"));
        }
    }

    pub fn unbind(){
        unsafe{
            gl::BindVertexArray(0);
        }
    }

    pub fn add_pointer(&self, ptr: &VertexPointer){
        unsafe{
            gl::VertexAttribPointer(ptr.location, ptr.size as GLint, ptr.type_value, match ptr.normalized {
                true => { gl::TRUE }
                _ => { gl::FALSE }
            }, ptr.raw_size as GLsizei, ptr.pointer as *const std::os::raw::c_void);
        }
    }

    pub fn enable_pointer(&self, ptr: u32){
        unsafe{
            gl::EnableVertexAttribArray(ptr);
        }
    }
}

impl Drop for Vao{
    fn drop(&mut self) {
        unsafe{
            gl::BindVertexArray(0);
            gl::DeleteVertexArrays(1, &mut self.buf.expect("Tried to drop non-existent VAO"));
        }
    }
}