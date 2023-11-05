use russimp;
use russimp::scene::PostProcess;
use crate::graphics;

pub fn load_mesh(file_name: &str) -> Vec<graphics::mesh::Mesh>{
    let mut meshes: Vec<graphics::mesh::Mesh> = std::vec![];

    let scene = russimp::scene::Scene::from_file(file_name,
    vec![PostProcess::CalculateTangentSpace, PostProcess::Triangulate,
    PostProcess::JoinIdenticalVertices,
    PostProcess::SortByPrimitiveType,
    PostProcess::OptimizeMeshes])
        .expect("Couldn't load the scene!");
    for mesh in scene.meshes{
        let mut vertices: Vec<graphics::mesh::Vertex> = std::vec![];
        let mut indices : Vec<u32> = std::vec![];
        for vertex in mesh.vertices{
            vertices.push(graphics::mesh::Vertex{
                pos: glm::Vector3 {
                    x: (vertex.x),
                    y: (vertex.y),
                    z: (vertex.z),
                },
            });
        }

        for face in mesh.faces{
            for index in face.0{
                indices.push(index);
            }
        }


        meshes.push(
            graphics::mesh::Mesh::new()
            .create()
            .load_vertices(&vertices, &indices));
    }

    meshes
}