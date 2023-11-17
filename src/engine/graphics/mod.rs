use glm;
pub mod mesh;
pub mod camera;
pub mod window;
pub mod material;
pub mod mesh_renderer;

const CAMERA_BINDING: u32 = 0;
pub const MODEL_LOCATION: i32 = 0;
pub const DIFFUSE_LOCATION: i32 = 1;
pub const DIFFUSE_TEXTURE: u32 = 0;
pub const WINDOW_RESOLUTION: [u32;2] = [1600, 900];
pub const MAT4_IDENTITY: glm::Matrix4<f32> = glm::Matrix4{ c0: glm::Vector4{x: 1.0, y: 0.0, z: 0.0, w: 0.0},
                                                           c1: glm::Vector4{x: 0.0, y: 1.0, z: 0.0, w: 0.0},
                                                            c2: glm::Vector4{x: 0.0, y: 0.0, z: 1.0, w: 0.0},
                                                            c3: glm::Vector4{x: 0.0, y: 0.0, z: 0.0, w: 1.0}};
