mod ogl;
mod graphics;

use std::ops::Add;
use sdl2;

fn main() {
    let sdl2 = sdl2::init()
        .expect("Couldn't Init SDL2 (someone really fucked up)");

    let sdl2_video = sdl2.video()
        .expect("Couldn't create video subsystem");


    let gl_attr = sdl2_video.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 6);

    let sdl2_gl_window = sdl2_video.window("SCP Game", 1600, 900)
        .opengl()
        .position_centered()
        .build()
        .expect("Couldn't make OpenGL Window!!!");

    gl::load_with(|name| sdl2_video.gl_get_proc_address(name) as *const _);

    let _ogl_ctx = sdl2_gl_window.gl_create_context().expect("Couldn't create GL Context");

    let vertices: Vec<f32> = std::vec![-5.0, -5.0, 0.0,
                                            5.0, -5.0, 0.0,
                                            2.5, 5.0, 0.0];
    let indices: Vec<u32> = std::vec![0, 1, 2];

    let mut triangle = graphics::mesh::Mesh::new();
    triangle.create();
    triangle.load_vertices(&vertices, &indices);

    let basic_pipeline = ogl::shader::Shader::new(String::from("shaders/vertex.glsl"),
                                                            String::from("shaders/fragment.glsl"));

    let mut camera = graphics::camera::Camera::new();

    let mut events = sdl2.event_pump().expect("No Event Pump");

    'mainLoop: loop {
        for event in events.poll_iter(){
            match event {
                sdl2::event::Event::Quit {..} => { break 'mainLoop; }
                _ => {}
            }
        }
        camera.pos = camera.pos.add(glm::vec3(-0.1, 0.0, 0.0));
        camera.calculate_projection();
        camera.bind();
        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        basic_pipeline.use_shader();
        ogl::shader::Shader::uniform_mat4(graphics::MAT4_IDENTITY, graphics::MODEL_LOCATION);
        triangle.draw_all();

        sdl2_gl_window.gl_swap_window();
    }

    println!("Hello, world!");
}
