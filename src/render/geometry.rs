
use std::rc::Rc;
use pz5::Pz5Geometry;
use pz5::GeometryType;
use glium::index::PrimitiveType;
use object_pool::growable::{ID,Slot};


use super::Error;
use super::Window;
use super::Frame;
use super::ModelShader;
use super::{VBO,VBOTrait};
use super::vertex;

pub struct Geometry{
    pub id:ID,
    vbo:Box<VBOTrait>,
}

impl Slot for Geometry{
    fn set_id(&mut self,id:ID) {
        self.id=id;
    }

    fn get_id(&self) -> ID {
        self.id
    }
}

impl Geometry{
    pub fn new(
        geometry:Pz5Geometry,
        geometry_type:GeometryType,
        vertex_format:String,
        shader:Rc<ModelShader>,
        window:&Window
    ) -> Result<Self, Error> {
        let primitive_type=match geometry_type{
            GeometryType::Points => PrimitiveType::Points,
            GeometryType::Lines => PrimitiveType::LinesList,
            GeometryType::Triangles => PrimitiveType::TrianglesList,
        };

        let vbo:Box<VBOTrait>=match vertex_format.as_str() {
            "VERTEX:(X:f32,Y:f32)" => Box::new( VBO::<vertex::VertexP2>::new(
                geometry,primitive_type,shader,window
            )? ),
            "VERTEX:(X:f32,Y:f32,Z:f32)" => Box::new( VBO::<vertex::VertexP3>::new(
                geometry,primitive_type,shader,window
            )? ),
            "VERTEX:(X:f32,Y:f32,Z:f32) NORMAL:(X:f32,Y:f32,Z:f32)" => Box::new( VBO::<vertex::VertexP3N3>::new(
                geometry,primitive_type,shader,window
            )? ),
            _ => return Err(Error::NoShaderProgram(String::from("aaa"))),
        };

        let geometry=Geometry{
            id:ID::zeroed(),
            vbo:vbo,
        };

        Ok(geometry)
    }

    pub fn render(&self, frame:&mut Frame) -> Result<(),Error> {
        self.vbo.render(frame)?;

        Ok(())
    }
}
