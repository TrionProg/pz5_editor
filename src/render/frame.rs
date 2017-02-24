use std;
use glium;

use glium::index::PrimitiveType;

pub struct ObjectFrame<'a>{
    pub target:glium::Frame,
    pub perspective_matrix:[[f32; 4]; 4],
    pub view_matrix:[[f32; 4]; 4],
    pub draw_parameters:glium::draw_parameters::DrawParameters<'a>,
}

impl<'a> ObjectFrame<'a>{
    pub fn finish(self){//TODO:Error
        self.target.finish().unwrap();
    }
}
