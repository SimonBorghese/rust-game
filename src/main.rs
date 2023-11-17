mod engine;
mod game;

use std::ops::Add;
use sdl2;
use gl;
use engine::*;
use crate::game::gameobject::GameObject;
use rapier3d::prelude::*;

fn main() {
    let mut window = graphics::window::Window::new().create();

    let mut physics = engine::physics::scene::Scene::new().create();

    let mut floor = physics::collider::ColliderMesh::new()
        .create_cuboid(100.0, 100.0, 0.1)
        .add_to_scene(&mut physics);

    let mut alien = game::invaders::Alien::new();
    alien.instantiate(&mut physics);

    let basic_pipeline = ogl::shader::Shader::new(String::from("shaders/vertex.glsl"),
                                                            String::from("shaders/fragment.glsl"));

    let mut camera = graphics::camera::Camera::new();

    let mut timer = window.sdl
        .timer()
        .expect("Couldn't get timer!");


    let mut last_time = timer.ticks();

    let mut aliens = std::vec![alien];

    'mainLoop: loop {
        timer.delay(16 - (timer.ticks() - last_time));
        let deltatime = (timer.ticks() - last_time) as f32 / 1000.0;
        //println!("Deltatime: {}", deltatime);

        for event in window.get_event_pump(){
            match event {
                sdl2::event::Event::Quit {..} => { break 'mainLoop; }
                sdl2::event::Event::KeyDown {scancode, ..} => {
                    match scancode.expect("Couldn't get scancode!"){
                        sdl2::keyboard::Scancode::Space{..} => {aliens.pop().expect("Couldn't pop alien");}
                        sdl2::keyboard::Scancode::K {..} => {
                            aliens.push(game::invaders::Alien::new());
                            aliens
                                .last_mut()
                                .expect("Couldn't get alien")
                                .instantiate(&mut physics);

                            aliens
                                .last_mut()
                                .unwrap()
                                .model
                                .as_mut()
                                .expect("Couldn't get model")
                                .transform
                                .pos.x = camera.pos.x;
                        }
                        _ => {}
                    }
                }
                sdl2::event::Event::MouseMotion {xrel,yrel,..} => {camera.rotate(xrel, -yrel);}
                _ => {}
            }
        }

        for key in window.get_scancode_iterator().scancodes(){
            if !key.1 {
                continue;
            }
            match key.0{
                sdl2::keyboard::Scancode::W => { camera.move_forward(deltatime); }
                sdl2::keyboard::Scancode::S => { camera.move_forward(-deltatime); }
                sdl2::keyboard::Scancode::A => { camera.move_right(-deltatime); }
                sdl2::keyboard::Scancode::D => { camera.move_right(deltatime); }

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

        for a in &mut aliens{
            a.loop_frame(deltatime);
            a.loop_physics(&mut physics);
        }
        physics.signal_existence(floor.get_handle());
        physics.step(deltatime);
        window.flip();

        last_time = timer.ticks();
    }

    println!("Hello, world!");
}
