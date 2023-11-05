use gl;
use sdl2;

pub struct Window{
    pub sdl: sdl2::Sdl,
    video: Option<sdl2::VideoSubsystem>,
    event: Option<sdl2::EventPump>,
    window: Option<sdl2::video::Window>,
    context: Option<sdl2::video::GLContext>,
}

impl Window{
    pub fn new() -> Window{
        Window{
            sdl: sdl2::init().expect("Couldn't init SDL in Window!!"),
            video: None,
            event: None,
            window: None,
            context: None,
        }
    }

    pub fn create(mut self) -> Window{
        let video = self.sdl.video().expect("Couldn't get SDL2 video!");

        let gl_attr = &video.gl_attr();
        gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
        gl_attr.set_context_version(4, 6);

        self.window = Some(video.window("SCP Game", crate::graphics::WINDOW_RESOLUTION[0],
        crate::graphics::WINDOW_RESOLUTION[1])
            .opengl()
            .position_centered()
            .build().expect("Couldn't build window!"));

        // Context must be assigned so it doesn't drop
        self.context = Some(self.window.as_mut()
            .unwrap()
            .gl_create_context().expect("Couldn't create GL Context"));

        gl::load_with(|name| video.gl_get_proc_address(name) as *const _);

        video.gl_set_swap_interval(-1).expect("Couldn't set swap interval");

        unsafe{
            gl::Viewport(0, 0,
                         crate::graphics::WINDOW_RESOLUTION[0] as gl::types::GLsizei,
                         crate::graphics::WINDOW_RESOLUTION[1] as gl::types::GLsizei);

            gl::Enable(gl::DEPTH_TEST);
        }

        self.video = Some(video);
        self.event = Some(self.sdl.event_pump().expect("Couldn't get event pump!"));

        self
    }

    pub fn get_event_pump(&mut self) -> sdl2::event::EventPollIterator{
        self.event.as_mut().expect("Couldn't poll events").poll_iter()
    }

    pub fn flip(&self){
        self.window.as_ref().expect("Couldn't get Window!").gl_swap_window();
    }
}