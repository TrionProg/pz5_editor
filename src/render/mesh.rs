use std;
use glium;

use std::rc::Rc;

use glium::Program;

use super::LOD;

pub trait MeshTrait{
    fn display(&self) -> bool;
    fn render(&self,distance:f32);
}

pub struct Mesh<V:glium::vertex::Vertex>{
    display:bool,
    shader:Rc<Program>,
    lods:Vec<LOD<V>>,
}

impl<V:glium::vertex::Vertex> MeshTrait for Mesh<V>{
    fn display(&self) -> bool{
        self.display
    }

    fn render(&self, distance:f32){

    }
}
