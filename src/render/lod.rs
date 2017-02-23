use std;
use glium;
use render;

use std::rc::Rc;
use glium::VertexBuffer;

use Render;

pub trait LODTrait{
    fn render(&self, render:&Render);
}


pub struct LOD<V:glium::vertex::Vertex>{
    vbo:VertexBuffer<V>,
    program:Rc<render::Program>,
    primitive_type:glium::index::PrimitiveType,
}

impl<V:glium::vertex::Vertex> LODTrait for LOD<V>{
    fn render(&self, render:&Render){
        /*target.draw(&vertex_buffer,
                    &glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
                    &program, &uniforms, &params).unwrap();
        */
        //target in render required, uniforms, params
    }
}
