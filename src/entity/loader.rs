use russimp;
use russimp::scene::PostProcess;
use crate::graphics;

pub fn load_mesh(file_name: &str) -> Vec<graphics::mesh_renderer::MeshRenderer>{
    let mut meshes: Vec<graphics::mesh_renderer::MeshRenderer> = std::vec![];

    let scene = russimp::scene::Scene::from_file(file_name,
    vec![PostProcess::CalculateTangentSpace, PostProcess::Triangulate,
    PostProcess::JoinIdenticalVertices,
    PostProcess::SortByPrimitiveType,
    PostProcess::OptimizeMeshes])
        .expect("Couldn't load the scene!");

    for mesh in scene.meshes{
        let mut vertices: Vec<graphics::mesh::Vertex> = std::vec![];
        let mut indices : Vec<u32> = std::vec![];
        for v in 0..mesh.vertices.len(){
            let vex = mesh.vertices.get(v).expect("Missing Vertex!");
            let norm = mesh.normals.get(v).expect("Missing Normal!");
            let tex = mesh.texture_coords
                .get(0)
                .expect("No texture coords!").as_ref()
                .expect("Couldn't get texture coord pointer!")
                .get(v)
                .expect("Couldn't get texture coords");

            vertices.push(graphics::mesh::Vertex::new()
                .pos(vex.x, vex.y, vex.z)
                .norm(norm.x, norm.y, norm.z)
                .tex(tex.x, tex.y));
        }

        for face in mesh.faces{
            for index in face.0{
                indices.push(index);
            }
        }


        meshes.push(graphics::mesh_renderer::MeshRenderer::new()
                        .add_mesh(graphics::mesh::Mesh::new().create()
            .load_vertices(&vertices, &indices))
            .add_material(
                graphics::material::Material::new()
                    .load_texture(
                        match scene.materials[mesh.material_index as usize].textures
                        .get(&russimp::material::TextureType::Diffuse)
                        {
                            None => { String::from("missing.png") }
                            Some(tex) => {
                                tex.borrow().filename.clone()
                            }
                        })
            ));
    }

    meshes
}