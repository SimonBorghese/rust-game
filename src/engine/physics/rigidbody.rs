use rapier3d::na::Vector3;
use rapier3d::prelude::*;
use super::*;

pub struct RigidBodyMesh{
    body: Option<RigidBody>,
    handle: Option<RigidBodyHandle>
}

impl RigidBodyMesh{
    pub fn new() -> RigidBodyMesh{
        RigidBodyMesh{
            body: None,
            handle: None
        }
    }

    pub fn create_dynamic_body(mut self, position: Vector3<f32>) -> Self{
        self.body = Some(RigidBodyBuilder::dynamic()
            .translation(position)
            .build());
        self
    }

    pub fn add_to_scene(mut self, scene: &mut scene::Scene) -> Self{
        self.handle = Some(scene.rigidbody_set
            .as_mut()
            .expect("Got invalid scene!")
            .insert(self.body.clone()
                .expect("Tried to add to scene without body!")));
        self
    }

    pub fn get_handle(&self) -> RigidBodyHandle{
        self.handle
            .as_ref()
            .expect("Tried to clone handle without an existing one! Did you add to the scene?")
            .clone()
    }

    pub fn get_body<'a>(&'a self, scene: &'a scene::Scene) -> &RigidBody{
        &scene.rigidbody_set
            .as_ref()
            .expect("Got invalid scene!")
            [self.handle.expect("Unable to get handle!")]
    }

}