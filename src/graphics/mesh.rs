use crate::ogl;
use crate::ogl::vao::VertexPointer;

pub struct Mesh{
    vao: Option<ogl::vao::Vao>,
    vbo: Option<ogl::buffer::Buffer>,
    ebo: Option<ogl::buffer::Buffer>,
    num_vertices: usize
}

impl Mesh{
    pub fn new() -> Mesh{
        Mesh{
            vao: None,
            vbo: None,
            ebo: None,
            num_vertices: 0,
        }
    }
    pub fn create(&mut self){
        self.vao = Some(ogl::vao::Vao::new());
        self.vbo = Some(ogl::buffer::Buffer::new().create());
        self.ebo = Some(ogl::buffer::Buffer::new().create());
        self.num_vertices = 0;
    }

    pub fn load_vertices(&mut self, vertices: &Vec<f32>, indices: &Vec<u32>){
        let vao = self.vao.as_mut().expect("No VAO");
        let vbo = self.vbo.as_mut().expect("No VBO");
        let ebo = self.ebo.as_mut().expect("No EBO");
        vao.bind();
        vbo.bind(gl::ARRAY_BUFFER);
        ebo.bind(gl::ELEMENT_ARRAY_BUFFER);

        self.num_vertices = indices.len();

        vbo.data(gl::ARRAY_BUFFER, Some(vertices.as_ptr()), Some(vertices.len()));
        ebo.data(gl::ELEMENT_ARRAY_BUFFER, Some(indices.as_ptr()), Some(indices.len()));

        vao.add_pointer(VertexPointer::new()
            .location(0)
            .num_values(3)
            .type_value(gl::FLOAT)
            .raw_size(std::mem::size_of::<f32>() * 3)
            .pointer(0));
        vao.enable_pointer(0);

        ogl::vao::Vao::unbind();
        ogl::buffer::Buffer::unbind(gl::ARRAY_BUFFER);
        ogl::buffer::Buffer::unbind(gl::ELEMENT_ARRAY_BUFFER);
    }

    pub fn draw_all(&self){
        self.vao.as_ref().expect("No VAO").bind();
        unsafe{
            gl::DrawElements(gl::TRIANGLES,
                             self.num_vertices as gl::types::GLsizei,
                             gl::UNSIGNED_INT,
                             0 as *const std::os::raw::c_void);
        }
    }
}

