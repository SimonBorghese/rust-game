use super::physics::*;
pub mod loader;
pub mod transform;

pub struct PhysicsEntity{
    body: Option<rigidbody::RigidBodyMesh>,
    collider: Option<collider::ColliderMesh>
}
pub struct Entity{
    pub transform: transform::Transform,
    enable_render: bool,
    meshes: Option<Vec<crate::graphics::mesh_renderer::MeshRenderer>>,
    physics_element: Option<PhysicsEntity>
}

impl Entity{
    pub fn new() -> Entity{
        Entity{
            transform: transform::Transform::new(),
            enable_render: true,
            meshes: None,
            physics_element: None
        }
    }

    pub fn load_model(mut self, file_name: &str) -> Self{
        self.meshes = Some(loader::load_mesh(file_name));

        self
    }

    pub fn render(&self){
        if !self.enable_render { return; }

        crate::ogl::shader::Shader::uniform_mat4(self.transform.get_model(),
                                                 crate::graphics::MODEL_LOCATION);
        for mesh in self.meshes.as_ref().expect("No loaded Meshes!"){
            mesh.render();
        }
    }

    pub fn update_physics(&mut self, scene: &mut scene::Scene){
        let physics_element = self.physics_element.as_ref().expect("Tried to update physics without physics element");
        let collider = physics_element.collider.as_ref().expect("Tried to update physics without a collider!");
        let body = physics_element.body.as_ref().expect("Tried to update physics without body");
        scene.signal_existence(collider.get_handle());
        let real_body = body.get_body(scene);

        let real_position = real_body.translation();
        self.transform.pos = glm::vec3(real_position.x, real_position.y, real_position.z);
    }

    pub fn add_physics(mut self) -> Self{
        self.physics_element = Some(PhysicsEntity{
            body: None,
            collider: None,
        });

        self
    }

    pub fn add_collider(mut self, scene: &mut scene::Scene, shape: collider::ColliderMesh) -> Self{
        let physics = self.physics_element.as_mut()
            .expect("Tried to add collider without physics");
        let body = physics.body.as_mut()
            .expect("Tried to add collider without body");
        self.physics_element.as_mut().expect("Tried to add collider without physics")
            .collider = Some(shape.add_to_scene_with_body(scene, body));
        self
    }

    pub fn add_rigidbody(mut self, scene: &mut scene::Scene, body: rigidbody::RigidBodyMesh) -> Self{
        self.physics_element.as_mut().expect("Tried to add rigidbody without physics")
            .body = Some(body.add_to_scene(scene));
        self
    }

    pub fn enable_render(&mut self){
        self.enable_render = true;
    }
    pub fn disable_render(&mut self){
        self.enable_render = false;
    }
}