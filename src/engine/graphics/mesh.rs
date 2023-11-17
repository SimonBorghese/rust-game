use crate::ogl;
use crate::ogl::vao::VertexPointer;

#[repr(C)]
pub struct Vertex{
    pub pos: glm::Vec3,
    pub norm: glm::Vec3,
    pub tex: glm::Vec2
}
pub struct Mesh{
    vao: Option<ogl::vao::Vao>,
    vbo: Option<ogl::buffer::Buffer>,
    ebo: Option<ogl::buffer::Buffer>,
    num_vertices: usize
}

impl Vertex{
    pub fn new() -> Vertex{
        Vertex{
            pos: glm::vec3(0.0, 0.0, 0.0),
            norm: glm::vec3(0.0, 0.0, 0.0),
            tex: glm::vec2(0.0, 0.0)
        }
    }

    pub fn pos(mut self, x: f32, y: f32, z: f32) -> Vertex{
        self.pos = glm::vec3(x,y,z);

        self
    }

    pub fn norm(mut self, x: f32, y: f32, z: f32) -> Vertex{
        self.norm = glm::vec3(x,y,z);

        self
    }

    pub fn tex(mut self, x: f32, y: f32) -> Vertex{
        self.tex = glm::vec2(x,y);

        self
    }
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
    pub fn create(mut self) -> Mesh{
        self.vao = Some(ogl::vao::Vao::new());
        self.vbo = Some(ogl::buffer::Buffer::new().create());
        self.ebo = Some(ogl::buffer::Buffer::new().create());
        self.num_vertices = 0;

        self
    }

    pub fn load_vertices(mut self, vertices: &Vec<Vertex>, indices: &Vec<u32>) -> Mesh{
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
            .raw_size(std::mem::size_of::<Vertex>())
            .pointer(0));
        vao.add_pointer(VertexPointer::new()
            .location(1)
            .num_values(3)
            .type_value(gl::FLOAT)
            .raw_size(std::mem::size_of::<Vertex>())
            .pointer(std::mem::size_of::<f32>() * 3));
        vao.add_pointer(VertexPointer::new()
            .location(2)
            .num_values(2)
            .type_value(gl::FLOAT)
            .raw_size(std::mem::size_of::<Vertex>())
            .pointer(std::mem::size_of::<f32>() * 6));
        vao.enable_pointer(0);
        vao.enable_pointer(1);
        vao.enable_pointer(2);

        ogl::vao::Vao::unbind();
        ogl::buffer::Buffer::unbind(gl::ARRAY_BUFFER);
        ogl::buffer::Buffer::unbind(gl::ELEMENT_ARRAY_BUFFER);

        self
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

