use std;
use glium;
use glutin;

use glutin::ElementState;
use glutin::MouseButton;

use RenderError;
use Window;
use super::Input;

pub struct GUI{
    pub input:Input,
}

impl GUI{
    pub fn new(window:&Window) -> Result<Self,RenderError> {
        let gui=GUI{
            input:Input::new(),
        };

        Ok(gui)
    }

    pub fn on_mouse_move(&mut self, x:i32, y:i32){
        self.input.on_mouse_move(x,y);
    }

    pub fn on_mouse_button(&mut self, state:ElementState, button:MouseButton){
        self.input.on_mouse_button(state,button);
    }
}
/*


use std;
use glium;
use glutin;

use std::rc::Rc;
use std::path::Path;

use std::thread;
use std::time::{Duration, Instant};

use glutin::Event;
use glutin::ElementState;
use glutin::MouseButton;

use Error;
use Object;
use Render;
use ObjectFrame;

use super::Camera;
use super::Viewport;
use super::Input;

pub struct GUI{
    object:Option<Object>,
    render:Rc<Render>,
    camera:Camera,
    input:Input,
    exit:bool,
}

impl GUI {
    fn new() -> Result<Self,Error>{
        let render=Rc::new( Render::new()? );
        let camera=Camera::new(&render);

        Ok(
            GUI{
                object:None,
                render:render,
                camera:camera,
                input:Input::new(),
                exit:false,
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
/*
    pub fn get_object_frame(&self) -> Result<ObjectFrame, RenderError>{
        self.camera.update(window_width, window_height, &self.input);
        let (perspective_matrix, camera_matrix) = self.camera.get_matrixes();

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
*/
    fn on_window_resize(&mut self, window_width:u32, window_height:u32){
        //Rc::get_mut(&mut self.render).unwrap().window_width = window_width;
        //Rc::get_mut(&mut self.render).unwrap().window_height = window_height;

        self.camera.viewport=Viewport::configure(&self.render);
    }

    fn on_close(&mut self) -> bool{
        true
    }

    fn on_mouse_move(&mut self, x:i32, y:i32) {
        self.input.on_mouse_move(x,y);

        if self.input.middle_mouse_button==ElementState::Pressed {
            self.camera.rotate(&self.input);
        }
    }

    fn on_mouse_button(&mut self, state:ElementState, button:MouseButton){
        self.input.on_mouse_button(state, button);

        //click etc
    }

    fn render(&mut self){
    }

    fn process(&mut self) {
        let mut accumulator = Duration::new(0, 0);
        let mut previous_clock = Instant::now();

        loop {
            let events:Vec<Event>=self.render.display.poll_events().collect();

            for event in events.iter() {
                match *event{
                    Event::Resized(window_width, window_height) =>
                        self.on_window_resize(window_width, window_height),
                    Event::Closed => {
                        if self.on_close() {
                            return;
                        }
                    },
                    Event::MouseMoved( x,y ) =>
                        self.on_mouse_move(x,y),
                    Event::MouseInput( event, button ) =>
                        self.on_mouse_button(event, button),
                    Event::Refresh =>
                        self.render(),
                    _ => {}
                }
            }

            let now = Instant::now();
            accumulator += now - previous_clock;
            previous_clock = now;

            let fixed_time_stamp = Duration::new(0, 16666667);
            while accumulator >= fixed_time_stamp {
                accumulator -= fixed_time_stamp;

                // if you have a game, update the state here
            }

            thread::sleep(fixed_time_stamp - accumulator);
        }
    }

    pub fn run() -> Result<(),Error>{
        let mut gui=Self::new()?;

        gui.include_collada_model(std::path::Path::new("pz5_3.dae"))?;

        gui.process();

        Ok(())
    }
}

*/
