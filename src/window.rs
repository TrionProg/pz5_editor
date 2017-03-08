use std;
use glium;
use glutin;

use std::sync::Mutex;

use Error;

pub struct Window{
    pub display:glium::backend::glutin_backend::GlutinFacade,
    pub width:u32,
    pub height:u32,
}

impl Window{
    pub fn new(width:u32, height:u32) -> Result<Self,Error> {
        use glium::DisplayBuild;

        let display=glutin::WindowBuilder::new()
            .with_title(String::from("pz5 editor"))
            .with_dimensions(width,height)
            .with_gl(glutin::GlRequest::Latest)
            .with_depth_buffer(24)
            .build_glium()?;

        let render=Window{
            display:display,
            width:width,
            height:height,
        };

        Ok( render )
    }

    pub fn resize(&mut self, width:u32, height:u32) {
        self.width=width;
        self.height=height;
    }
}
