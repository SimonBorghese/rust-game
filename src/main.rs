mod ogl;
mod graphics;
mod entity;

use std::ops::Add;
use sdl2;
use gl;

fn main() {
    let mut window = graphics::window::Window::new().create();

    let monke = entity::Entity::new().load_model("monke.obj");

    let basic_pipeline = ogl::shader::Shader::new(String::from("shaders/vertex.glsl"),
                                                            String::from("shaders/fragment.glsl"));

    let mut camera = graphics::camera::Camera::new();

    let mut timer = window.sdl.timer().expect("Couldn't get timer!");

    let mut last_time = timer.ticks();
    'mainLoop: loop {
        timer.delay(16 - (timer.ticks() - last_time));
        let deltatime = (timer.ticks() - last_time) as f32 / 1000.0;
        println!("Deltatime: {}", deltatime);
        for event in window.get_event_pump(){
            match event {
                sdl2::event::Event::Quit {..} => { break 'mainLoop; }
                sdl2::event::Event::KeyDown {scancode, ..} => {
                    match scancode.expect("Couldn't get scancode!"){
                        sdl2::keyboard::Scancode::W => {camera.move_forward(deltatime)}
                        sdl2::keyboard::Scancode::S => {camera.move_forward(-deltatime)}
                        sdl2::keyboard::Scancode::A => {camera.move_right(-deltatime)}
                        sdl2::keyboard::Scancode::D => {camera.move_right(deltatime)}
                        _ => {}
                    }
                }
                sdl2::event::Event::MouseMotion {xrel,yrel,..} => {camera.rotate(xrel, -yrel);}
                _ => {}
            }
        }
        camera.calculate_projection();
        camera.bind();
        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        basic_pipeline.use_shader();
        ogl::shader::Shader::uniform_mat4(graphics::MAT4_IDENTITY, graphics::MODEL_LOCATION);
        monke.render();
        window.flip();

        last_time = timer.ticks();
    }

    println!("Hello, world!");
}
