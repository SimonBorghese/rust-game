use std::ops::Add;
use gl;
use glm;
use glm::SignedNum;
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
    pub sens: f32,
    pub speed: f32,
    center: glm::Vec3,
    up: glm::Vec3,
    bind: CameraMatrix,
    buf: ogl::buffer::Buffer
}

impl Camera{
    pub fn new() -> Camera{
        Camera{
            pos: glm::vec3(0.0, 0.0, 5.0),
            rot: glm::vec3(0.0, 270.0, 0.0),
            center: glm::vec3(0.0, 0.0, -1.0),
            up: glm::vec3(0.0, 1.0, 0.0),
            bind: CameraMatrix {
                projection: MAT4_IDENTITY,
                view: MAT4_IDENTITY
            },
            buf: ogl::buffer::Buffer::new().create(),
            sens: 0.1,
            speed: 10.0
        }
    }

    pub fn rotate(&mut self, xrel: i32, yrel: i32){
        self.rot = self.rot.add(
            glm::vec3(yrel as f32 * self.sens,
                      xrel as f32 * self.sens, 0.0));

        if self.rot.x.abs() > 89.0{
            self.rot.x = 89.0 * (self.rot.x / self.rot.x.abs());
        }

        self.center = glm::normalize(
            glm::vec3(
                glm::cos(glm::radians(self.rot.y)) * glm::cos(glm::radians(self.rot.x)),
                glm::sin(glm::radians(self.rot.x)),
                glm::sin(glm::radians(self.rot.y)) * glm::cos(glm::radians(self.rot.x))
            )
        );
    }

    pub fn move_forward(&mut self, deltatime: f32){
        self.pos = self.pos.add(self.center * self.speed * deltatime);
    }

    pub fn move_right(&mut self, deltatime: f32){
        self.pos = self.pos.add(glm::cross(self.center, self.up) * self.speed * deltatime);
    }

    pub fn calculate_projection(&mut self) -> &mut Camera{
        self.bind.projection = glm::ext::perspective(glm::radians(89.9),
                                                    1600.0 / 900.0,
                                                    0.1,
                                                    1000.0);
        self.bind.view = glm::ext::look_at(self.pos,
                                            self.pos.add(self.center),
                                            self.up);

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