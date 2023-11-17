use crate::graphics::{DIFFUSE_TEXTURE};
use crate::ogl;
pub struct Material{
    texture: Option<ogl::texture::Texture>
}

impl Material{
    pub fn new() -> Material{
        Material{
            texture: None
        }
    }

    pub fn load_texture(mut self, file_path: String) -> Material{
        let img = image::io::Reader::open(file_path)
            .expect("Unable to open image!")
            .decode()
            .expect("Unable to decode image!");
        self.texture = Some(ogl::texture::Texture::new()
            .create()
            .load(img.width(), img.height(), img.into_rgba8().as_ptr()));
        self
    }

    pub fn bind(&self){
        self.texture.as_ref()
            .expect("Tried to bind material without diffuse texture!")
            .bind(DIFFUSE_TEXTURE);
    }

}