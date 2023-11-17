use crate::graphics;
use crate::graphics::{DIFFUSE_LOCATION, DIFFUSE_TEXTURE};
use crate::ogl;
pub struct MeshRenderer{
    mesh: Option<graphics::mesh::Mesh>,
    material: Option<graphics::material::Material>
}

impl MeshRenderer{
    pub fn new() -> MeshRenderer{
        MeshRenderer{
            mesh: None,
            material: None
        }
    }

    pub fn add_mesh(mut self, mesh: graphics::mesh::Mesh) -> MeshRenderer{
        self.mesh = Some(mesh);

        self
    }

    pub fn add_material(mut self, material: graphics::material::Material) -> MeshRenderer{
        self.material = Some(material);

        self
    }

    pub fn render(&self){
        self.material.as_ref().expect("Tried to render without material!").bind();
        ogl::shader::Shader::uniform_int(DIFFUSE_TEXTURE as _, DIFFUSE_LOCATION);

        self.mesh.as_ref().expect("Tried to render without mesh!").draw_all();
    }
}