use std;
use glium;
use cgmath;

use glium::index::PrimitiveType;
use glium::draw_parameters::DrawParameters;
use glium::Surface;

use cgmath::Matrix4;

use Window;
use Camera;

pub struct RenderFrame<'a>{
    pub target:glium::Frame,
    pub perspective_matrix:Matrix4<f32>,//[[f32; 4]; 4],
    //pub view_matrix:[[f32; 4]; 4],
    pub camera_matrix:Matrix4<f32>,
    pub draw_parameters:DrawParameters<'a>,
}

impl<'a> RenderFrame<'a>{
    pub fn new( camera:&Camera, window:&Window ) -> Option<Self> {
        let (perspective_matrix, camera_matrix) = match camera.get_matrixes() {
            Some( matrixes ) => matrixes,
            None => return None,
        };

        let mut target = window.display.draw();
        target.clear_color_and_depth((0.2, 0.2, 0.2, 0.0), 1.0);

        let draw_parameters=DrawParameters {
            depth: glium::Depth {
                test: glium::DepthTest::IfLess,
                write: true,
                .. Default::default()
            },
            .. Default::default()
        };

        let frame=RenderFrame{
            target:target,
            perspective_matrix:perspective_matrix,
            camera_matrix:camera_matrix,
            draw_parameters:draw_parameters,
        };

        Some( frame )
    }

    pub fn finish(self){//TODO:Error
        self.target.finish().unwrap();
    }
}
