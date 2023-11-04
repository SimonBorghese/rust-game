use gl;
use sdl2;

struct Window<'a>{
    sdl: sdl2::Sdl,
    video: Option<&'a sdl2::VideoSubsystem>,
    event: Option<&'a sdl2::EventPump>
}

impl<'a> Window<'a>{
    pub fn new() -> Window<'a>{
        Window{
            sdl: sdl2::init().expect("Couldn't init SDL in Window!!"),
            video: None,
            event: None
        }
    }
}
