use std::ops::Add;
use glm;
use crate::graphics;

pub struct Transform{
    pub pos: glm::Vec3,
    pub rot: glm::Vec3,
    pub scale: glm::Vec3
}

impl Transform{
    pub fn new() -> Transform{
        Transform{
            pos: glm::vec3(0.0, 0.0, 0.0),
            rot: glm::vec3(0.0, 0.0, 0.0),
            scale: glm::vec3(1.0, 1.0, 1.0)
        }
    }

    pub fn position(mut self, pos: glm::Vec3) -> Transform{
        self.pos = pos;

        self
    }

    pub fn rotation(mut self, rot: glm::Vec3) -> Transform{
        self.rot = rot;
        self
    }

    pub fn scale(mut self, scale: glm::Vec3) -> Transform{
        self.scale = scale;
        self
    }

    pub fn translate(&mut self, offset: glm::Vec3){
        self.pos = self.pos.add(offset);
    }

    pub fn rotate(&mut self, rotation: glm::Vec3){
        self.rot = self.rot.add(rotation);
    }

    pub fn add_scale(&mut self, scale: glm::Vec3){
        self.scale = self.scale.add(scale);
    }

    pub fn get_pos(&self) -> glm::Vec3{
        self.pos
    }
    pub fn get_rot(&self) -> glm::Vec3{
        self.rot
    }
    pub fn get_scale(&self) -> glm::Vec3{
        self.scale
    }

    pub fn set_pos(&mut self, pos: glm::Vec3){
        self.pos = pos;
    }

    pub fn set_rot(&mut self, rot: glm::Vec3){
        self.rot = rot;
    }

    pub fn set_scale(&mut self, scale: glm::Vec3){
        self.scale = scale;
    }

    pub fn get_model(&self) -> glm::Mat4{
        let mut model = graphics::MAT4_IDENTITY;

        model = glm::ext::translate(&model, self.pos);
        model = glm::ext::rotate(&model, glm::radians(self.rot.x), glm::vec3(1.0, 0.0, 0.0));
        model = glm::ext::rotate(&model, glm::radians(self.rot.y), glm::vec3(0.0, 1.0, 0.0));
        model = glm::ext::rotate(&model, glm::radians(self.rot.z), glm::vec3(0.0, 0.0, 1.0));
        model = glm::ext::scale(&model, self.scale);

        model
    }
}