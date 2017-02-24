use std;
use glium;
use glutin;

use std::rc::Rc;
use std::path::Path;

use support;
use support::camera::CameraState;

use Error;
use Object;
use Render;
use ObjectFrame;

pub struct GUI{
    object:Option<Object>,
    render:Rc<Render>,
    camera:CameraState,
}

impl GUI {
    fn new() -> Result<Self,Error>{
        let render=Rc::new( Render::new()? );

        Ok(
            GUI{
                object:None,
                render:render,
                camera:CameraState::new(),
            }
        )
    }

    pub fn include_collada_model(&mut self, file_name:&Path) -> Result<(),Error> {
        if self.object.is_none() {
            self.object=Some( Object::empty( Some(self.render.clone()) ) );
        }

        match self.object {
            Some( ref mut object ) => {object.include_collada_model(file_name)?;},
            None => {},
        }

        Ok(())
    }

    pub fn get_object_frame(&self) -> ObjectFrame{
        use glium::Surface;

        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::DepthTest::IfLess,
                write: true,
                .. Default::default()
            },
            .. Default::default()
        };

        let mut target = self.render.display.draw();
        target.clear_color_and_depth((0.0, 0.0, 0.0, 0.0), 1.0);

        ObjectFrame{
            target:target,
            perspective_matrix:self.camera.get_perspective(),
            view_matrix:self.camera.get_view(),
            draw_parameters:params,
        }
    }

    pub fn process() -> Result<(),Error>{
        let mut gui=Self::new()?;

        gui.include_collada_model(std::path::Path::new("pz5_3.dae"))?;

        support::start_loop(|| {
            gui.camera.update();

            {
                let mut frame=gui.get_object_frame();

                match gui.object{
                    Some( ref object ) => object.render(&mut frame).unwrap(),
                    None => {},
                }

                frame.finish();
            }

            // polling and handling the events received by the window
            for event in gui.render.display.poll_events() {
                match event {
                    glutin::Event::Closed => return support::Action::Stop,
                    ev => gui.camera.process_input(&ev),
                }
            }
            /*
            // polling and handling the events received by the window
            for event in display.poll_events() {
                match event {
                    glutin::Event::Closed => return support::Action::Stop,
                    _ => ()
                }
            }
            */

            support::Action::Continue
        });

        Ok(())
    }

}
