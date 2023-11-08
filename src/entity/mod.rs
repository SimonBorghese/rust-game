pub mod loader;

pub struct Entity{
    meshes: Option<Vec<crate::graphics::mesh_renderer::MeshRenderer>>
}

impl Entity{
    pub fn new() -> Entity{
        Entity{
            meshes: None
        }
    }

    pub fn load_model(mut self, file_name: &str) -> Entity{
        self.meshes = Some(loader::load_mesh(file_name));

        self
    }

    pub fn render(&self){
        for mesh in self.meshes.as_ref().expect("No loaded Meshes!"){
            mesh.render();
        }
    }
}