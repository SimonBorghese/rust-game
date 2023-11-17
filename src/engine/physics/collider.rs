use rapier3d::prelude::*;
use super::*;

pub struct ColliderMesh{
    shape: Option<Collider>,
    handle: Option<ColliderHandle>
}

impl ColliderMesh{
    pub fn new() -> ColliderMesh{
        ColliderMesh{
            shape: None,
            handle: None
        }
    }

    pub fn create_cuboid(mut self, width: f32, length: f32, height: f32) -> Self{
        self.shape = Some(ColliderBuilder::cuboid(width, height, length).build());
        self
    }

    pub fn add_to_scene(mut self, scene: &mut scene::Scene) -> Self{
        self.handle = Some(scene.collider_set
            .as_mut()
            .expect("Got invalid scene!")
            .insert(self.shape.clone()
                .expect("Tried to add to scene without collider!")));

        self
    }

    pub fn get_handle(&self) -> ColliderHandle{
        self.handle
            .as_ref()
            .expect("Tried to clone handle without an existing one! Did you add to the scene?")
            .clone()
    }

    pub fn add_to_scene_with_body(mut self, scene: &mut scene::Scene, body: &rigidbody::RigidBodyMesh) -> Self{
        self.handle = Some(scene.collider_set
            .as_mut()
            .expect("Got invalid scene!")
                               .insert_with_parent(self.shape.clone()
                                   .expect("Tried to insert without a collider"),
                               body.get_handle(), &mut scene.rigidbody_set.as_mut()
                                       .expect("Got invalid scene!")));
        self
    }

}