use gl;
use gl::types::*;


pub struct Buffer {
    pub buf: Option<GLuint>,
    current_bind: Option<gl::types::GLenum>,
    max_size: usize
}

impl<'a> Buffer{
    
    pub fn new() -> Buffer{
        Buffer{
            buf: None,
            current_bind: None,
            max_size: 0,
        }
    }
    pub fn create(mut self) -> Buffer{
        unsafe{
            let mut buf:u32 = 0;
            gl::GenBuffers(1, &mut buf);
            self.buf = Some(buf);
        }

        self
    }

    pub fn bind(&mut self, buffer: GLenum){
        unsafe{
            gl::BindBuffer(buffer,
                           self.buf.expect("Non-existent buffer"));
        }
        self.current_bind = Some(buffer);
    }

    pub fn bind_base(&self, buffer_type: gl::types::GLenum, index: u32){
        unsafe{
            gl::BindBufferBase(buffer_type, index, self.buf.expect("No Buffer"));
        }
    }

    pub fn unbind(buffer: GLenum){
        unsafe {
            gl::BindBuffer(buffer, 0);
        }
    }
    pub fn data<T>(&mut self, buffer_type: gl::types::GLenum, data: Option<*const T>, length: Option<usize>){
        unsafe{
            gl::BufferData(
                buffer_type,
                (length.expect("Unexpected Length!") * std::mem::size_of::<T>()) as GLsizeiptr,
                data.expect("Got invalid data!") as *const std::os::raw::c_void,
                gl::STREAM_DRAW
            );
        }
        self.max_size = length.unwrap() * std::mem::size_of::<T>();
    }

    pub fn sub_data<T>(&mut self, data: Option<*const T>, offset: Option<GLsizeiptr>, length: Option<usize>){
        assert!(length.expect("No Length Provided!") <= self.max_size,
                    "ATTEMPTED TO UPDATE BUFFER WITH TOO MUCH SIZE!");
        unsafe{
            gl::NamedBufferSubData(
                self.buf.expect("Found unreal buffer!"),
                offset.unwrap_or(0),
                length.unwrap() as GLsizeiptr,
                data.expect("Got invalid data!") as *const std::os::raw::c_void
            );
        }
    }
}

impl Drop for Buffer{
    fn drop(&mut self) {
        unsafe{
            gl::DeleteBuffers(1, &self.buf.expect("Dropped non-existent buffer!"));
            println!("Dropping a buffer!");
        }
    }
}