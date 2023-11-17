use crate::engine::entity;
use crate::physics::*;
use rapier3d::prelude::*;
pub struct Alien{
    pub model: Option<entity::Entity>
}

impl Alien{
    pub fn new() -> Alien{
        Alien{
            model: None
        }
    }

    pub fn instantiate(&mut self, scene: &mut scene::Scene) {
        self.model = Some(entity::Entity::new().load_model("monke.obj")
            .add_physics()
            .add_rigidbody(scene,
                rigidbody::RigidBodyMesh::new()
                    .create_dynamic_body(vector![0.0, 10.0, 0.0])
            )
            .add_collider(scene,
                collider::ColliderMesh::new()
                    .create_cuboid(1.0, 1.0, 1.0)
            ));
    }

    pub fn loop_frame(&mut self, dt: f32) {
        self.model.as_ref().expect("Tried to loop without model!").render();
    }

    pub fn loop_physics(&mut self, scene: &mut scene::Scene){
        self.model.as_mut().expect("Tried to loop physics without model")
            .update_physics(scene);
    }

    pub fn destroy(&mut self) {
        self.model.as_mut().expect("Tried to destroy without model!").disable_render();
    }
}


impl Drop for Alien{
    fn drop(&mut self) {
        println!("Dropping an alien!");
    }
}