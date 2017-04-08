use std;
use glium;
use cgmath;

use glium::index::PrimitiveType;
use glium::draw_parameters::DrawParameters;
use glium::Surface;

use cgmath::Matrix4;

use Window;
use Camera;
use Storage;

pub struct RenderFrame<'a>{
    pub target:glium::Frame,
    pub perspective_matrix:Matrix4<f32>,//[[f32; 4]; 4],
    //pub view_matrix:[[f32; 4]; 4],
    pub camera_matrix:Matrix4<f32>,
    pub draw_parameters:DrawParameters<'a>,
    pub storage:&'a Storage,
}

impl<'a> RenderFrame<'a>{
    pub fn new( camera:&Camera, window:&Window, storage:&'a Storage ) -> Option<Self> {
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
            line_width: Some(1.0),
            .. Default::default()
        };

        let frame=RenderFrame{
            target:target,
            perspective_matrix:perspective_matrix,
            camera_matrix:camera_matrix,
            draw_parameters:draw_parameters,
            storage:storage,
        };

        Some( frame )
    }

    pub fn skeleton_mode(&mut self) {
        self.draw_parameters = DrawParameters {
            depth: glium::Depth {
                test: glium::DepthTest::Overwrite,
                write: true,
                .. Default::default()
            },
            line_width: Some(4.0),
            point_size: Some(6.0),
            .. Default::default()
        };
    }

    pub fn finish(self){//TODO:Error
        self.target.finish().unwrap();
    }
}
