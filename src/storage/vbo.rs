use std;
use pz5;
use glium;
use render;

use std::rc::Rc;
use glium::VertexBuffer;
use pz5::GeometryType;
use glium::index::PrimitiveType;
use pz5::Pz5Geometry;

use cgmath::Matrix4;

use RenderError;
use RenderFrame;
use Window;

use super::ModelShader;


pub trait VBOTrait{
    fn render(&self, frame:&mut RenderFrame, mesh_matrix:&Matrix4<f32>) -> Result<(),RenderError>;
}

pub struct VBO<V:glium::vertex::Vertex>{
    vertex_buffer:VertexBuffer<V>,
    shader:Rc<ModelShader>,
    primitive_type:PrimitiveType,
}

impl<V:glium::vertex::Vertex> VBOTrait for VBO<V>{
    fn render(&self, frame:&mut RenderFrame, mesh_matrix:&Matrix4<f32>) -> Result<(),RenderError>{
        let uniforms = uniform! {
            perspective_matrix: Into::<[[f32; 4]; 4]>::into(frame.perspective_matrix),
            camera_matrix: Into::<[[f32; 4]; 4]>::into(frame.camera_matrix),
            mesh_matrix: Into::<[[f32; 4]; 4]>::into(*mesh_matrix),
        };

        use glium::Surface;

        frame.target.draw(&self.vertex_buffer,
            &glium::index::NoIndices(self.primitive_type),
            &self.shader.glium_program,
            &uniforms,
            &frame.draw_parameters
        )?;

        Ok(())
    }
}

impl<V:glium::vertex::Vertex> VBO<V> {
    pub fn new(
        geometry:Pz5Geometry,
        primitive_type:PrimitiveType,
        shader:Rc<ModelShader>,
        window:&Window
    ) -> Result<Self,RenderError> {
        println!("LOAD");
        let vertex_buffer=glium::VertexBuffer::new(&window.display, geometry.as_buf::<V>() )?;

        let vbo=VBO{
            vertex_buffer:vertex_buffer,
            shader:shader,
            primitive_type:primitive_type,
        };

        Ok(vbo)
    }
}

/*
impl<V:glium::vertex::Vertex> VBO<V>{
    pub fn new(
        window:&Window,
        shader:Rc<ModelShader>,
        vertex_format:&String,
        geometry:Pz5Geometry,
        geometry_type:GeometryType
    ) -> Result<Box<Self>,RenderError>{
        let primitive_type=match geometry_type{
            GeometryType::Points => PrimitiveType::Points,
            GeometryType::Lines => PrimitiveType::LinesList,
            GeometryType::Triangles => PrimitiveType::TrianglesList,
        };

        let program=match shaders.get(vertex_format){
            Some( program ) => program.clone(),
            None => return Err( Error::NoShaderProgram(vertex_format.clone()) ),
        };

        let vbo=glium::VertexBuffer::new(&render.display, geometry.as_buf::<V>() )?;

        let vbo=VBO{
            vbo:vbo,
            program:program,
            primitive_type:primitive_type,
        };

        Ok( Box::new(vbo) )
    }

    pub fn load_vbo(
        window:&Window,
        shaders:HashMap<String,Rc<ModelShader>>,

        vertex_format:String,
        geometry:Pz5Geometry,
        geometry_type:GeometryType,
    )
}
*/
