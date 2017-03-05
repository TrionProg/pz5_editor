use std;
use glium;
use cgmath;

use cgmath::{Vector2,Vector3,PerspectiveFov,Matrix4};
use cgmath::{vec2,vec3};
use glutin::ElementState;

use Error;
use Window;

use gui::Input;
use super::Viewport;

pub struct Camera{
    center_position: Vector3<f32>,
    angle: Vector2<f32>,
    distance:f32,

    pub viewport:Option<Viewport>,
}

impl Camera{
    pub fn new(window:&Window) -> Result<Self,Error>{
        let camera=Camera{
            center_position: vec3(0.0,0.0,0.0),
            angle: vec2(0.0,0.0),
            distance: 10.0,

            viewport:Viewport::configure(window),
        };

        Ok( camera )
    }

    /*

    pub fn rotate(&mut self, input:&Input){
        let mouse_move_x=match self.viewport {
            Some( ref viewport ) => input.mouse_move_x as f32 / viewport.width as f32,
            None => 0.0,
        };

        let mouse_move_y=match self.viewport {
            Some( ref viewport ) => input.mouse_move_y as f32 / viewport.height as f32,
            None => 0.0,
        };

        self.angle.x+=mouse_move_x*3.14*2.0;
        self.angle.y+=mouse_move_y*3.14*2.0;

        if self.angle.y< -3.14/2.0 {
            self.angle.y=-3.14/2.0;
        }

        if self.angle.y> 3.14/2.0 {
            self.angle.y=3.14/2.0;
        }

        println!("{} {}",self.angle.x, self.angle.y);
    }
    */

    /*
    pub fn get_matrixes(&self) -> (Matrix4, Matrix4) {
        let perspective=PerspectiveFov{
            fovy:Rad::from(3.141592 / 2.0),
            aspect:self.aspect_ratio,
            near:0.1,
            far:1000.0,
        };

        let perspective_matrix=Matrix4::from(perspective);

        //let camera_pos=vec3(0,0,self.distance)*
    }
    */
}
