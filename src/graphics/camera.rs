use gl;
use glm;
use crate::graphics::{CAMERA_BINDING, MAT4_IDENTITY};
use crate::ogl;
use crate::ogl::buffer;

struct CameraMatrix{
    projection: glm::Mat4,
    view: glm::Mat4
}

pub struct Camera{
    pub pos: glm::Vec3,
    pub rot: glm::Vec3,
    center: glm::Vec3,
    bind: CameraMatrix,
    buf: ogl::buffer::Buffer
}

impl Camera{
    pub fn new() -> Camera{
        Camera{
            pos: glm::vec3(0.0, 0.0, -15.0),
            rot: glm::vec3(0.0, 0.0, 0.0),
            center: glm::vec3(0.0, 0.0, -1.0),
            bind: CameraMatrix {
                projection: MAT4_IDENTITY,
                view: MAT4_IDENTITY
            },
            buf: ogl::buffer::Buffer::new().create()
        }
    }

    pub fn calculate_projection(&mut self) -> &mut Camera{
        self.bind.projection = glm::ext::perspective(glm::radians(89.9),
                                                    1600.0 / 900.0,
                                                    0.1,
                                                    1000.0);
        self.bind.view = glm::ext::look_at(self.pos,
                                            self.center,
                                            glm::vec3(0.0, 1.0, 0.0));

        self
    }

    pub fn bind(&mut self) -> &mut Camera{
        self.buf.bind(gl::UNIFORM_BUFFER);
        self.buf.data(gl::UNIFORM_BUFFER, Some(&self.bind),
                      Some(1));
        self.buf.bind_base(gl::UNIFORM_BUFFER, CAMERA_BINDING);
        buffer::Buffer::unbind(gl::UNIFORM_BUFFER);

        self
    }
}