use gl;
use gl::types::*;

enum GlBuffers {
    VertexBuffer
}

pub struct Buffer {
    pub buf: Option<GLuint>,
    current_bind: Option<GlBuffers>,
    max_size: GLsizeiptr
}

impl Buffer{
    pub fn new() -> Buffer{
        let mut buf = Buffer{
            buf: None,
            current_bind: None,
            max_size: 0
        };

        unsafe{
            gl::GenBuffers(1, &mut buf.buf.unwrap());
        }

        buf
    }

    fn type2enum(rs_type: &GlBuffers) -> GLenum{
        match rs_type{
            GlBuffers::VertexBuffer => { gl::ARRAY_BUFFER }
        }
    }

    pub fn bind(&mut self, buffer: Option<GlBuffers>){
        unsafe{
            gl::BindBuffer(Buffer::type2enum(&buffer.expect("No provided buffer!")),
                           self.buf.expect("Non-existent buffer"));
        }
        self.current_bind = buffer;
    }
    pub fn data(&mut self, data: Option<&std::ptr>, length: Option<GLsizeiptr>){
        unsafe{
            gl::NamedBufferData(
                self.buf.expect("Found unreal buffer!"),
                length.expect("Unexpected Length!"),
                data.expect("Got invalid data!"),
                gl::STREAM_DRAW
            );
        }
        self.max_size = length.unwrap();
    }

    pub fn sub_data(&mut self, data: Option<&std::ptr>, offset: Option<GLsizeiptr>, length: Option<GLsizeiptr>){
        assert!(length.expect("No Length Provided!") <= self.max_size,
                    "ATTEMPTED TO UPDATE BUFFER WITH TOO MUCH SIZE!");
        unsafe{
            gl::NamedBufferSubData(
                self.buf.expect("Found unreal buffer!"),
                offset.or_else(0).unwrap(),
                length.unwrap(),
                data.expect("Got invalid data!")
            );
        }
    }
}

impl Drop for Buffer{
    fn drop(&mut self) {
        unsafe{
            gl::DeleteBuffers(1, &self.buf.expect("Dropped non-existent buffer!"));
        }
    }
}