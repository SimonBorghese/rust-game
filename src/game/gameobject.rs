pub trait GameObject{
    fn instantiate(&mut self);

    fn loop_frame(&mut self, dt: f32);

    fn destroy(&mut self);
}
