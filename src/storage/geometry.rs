use std;
use pz5;
use glium;
use render;

use std::rc::Rc;
use glium::VertexBuffer;
use pz5::GeometryType;
use glium::index::PrimitiveType;
use pz5::Pz5Geometry;

use Error;
use Render;
use ObjectFrame;

pub trait LODTrait{
    fn render(&self, frame:&mut ObjectFrame) -> Result<(),glium::DrawError>;
}

pub struct LOD<V:glium::vertex::Vertex>{
    vbo:VertexBuffer<V>,
    program:Rc<render::Program>,
    primitive_type:PrimitiveType,
}

impl<V:glium::vertex::Vertex> LODTrait for LOD<V>{
    fn render(&self, frame:&mut ObjectFrame) -> Result<(),glium::DrawError>{

        let uniforms = uniform! {
            persp_matrix: frame.perspective_matrix,
            view_matrix: frame.view_matrix,
        };

        use glium::Surface;

        frame.target.draw(&self.vbo,
                    &glium::index::NoIndices(self.primitive_type),
                    &self.program.glium_program,
                    &uniforms,
                    &frame.draw_parameters
        )
    }
}

impl<V:glium::vertex::Vertex> LOD<V>{
    pub fn new(render:&Render, fvf_str:&String, geometry:Pz5Geometry, geometry_type:GeometryType) -> Result<Box<Self>,Error>{
        let primitive_type=match geometry_type{
            GeometryType::Points => PrimitiveType::Points,
            GeometryType::Lines => PrimitiveType::LinesList,
            GeometryType::Triangles => PrimitiveType::TrianglesList,
        };

        let program=match render.programs.get(fvf_str){
            Some( program ) => program.clone(),
            None => return Err( Error::NoShaderProgram(fvf_str.clone()) ),
        };

        let vbo=glium::VertexBuffer::new(&render.display, geometry.as_buf::<V>() )?;

        Ok( Box::new(
            LOD{
                vbo:vbo,
                program:program,
                primitive_type:primitive_type,
            }
        ) )
    }
}
