use std;
use glium;
use cgmath;

use cgmath::{Vector2,Vector3,Point3,PerspectiveFov,Matrix4,Basis3,Rotation3};
use cgmath::{vec2,vec3,rad};
use glutin::ElementState;

use Error;
use Window;

use gui::Input;
use super::Viewport;

pub struct Camera{
    center_position: Point3<f32>,
    angle: Vector2<f32>,
    distance:f32,
    camera_matrix:Matrix4<f32>,

    pub viewport:Option<Viewport>,
}

impl Camera{
    pub fn new(window:&Window) -> Result<Self,Error>{
        use cgmath::SquareMatrix;

        let mut camera=Camera{
            center_position: Point3::new(0.0,0.0,0.0),
            angle: vec2(0.0,0.0),
            distance: 10.0,
            camera_matrix:Matrix4::identity(),

            viewport:Viewport::configure(window),
        };

        camera.calc_matrix();

        Ok( camera )
    }

    pub fn resize(&mut self, window:&Window) {
        self.viewport=Viewport::configure(window);
    }

    pub fn rotate(&mut self, input:&Input){
        let mouse_move_x=match self.viewport {
            Some( ref viewport ) => input.mouse_move_x as f32 / viewport.width as f32,
            None => 0.0,
        };

        let mouse_move_y=match self.viewport {
            Some( ref viewport ) => input.mouse_move_y as f32 / viewport.height as f32,
            None => 0.0,
        };

        self.angle.y+=mouse_move_x*3.14*1.5;
        self.angle.x+=mouse_move_y*3.14;

        if self.angle.x< -3.14/2.0 {
            self.angle.x=-3.14/2.0;
        }

        if self.angle.x> 3.14/2.0 {
            self.angle.x=3.14/2.0;
        }

        self.calc_matrix();

        println!("{} {}",self.angle.x, self.angle.y);
    }

    fn calc_matrix(&mut self) {
        use cgmath::ApproxEq;
        use cgmath::Rotation;
        use cgmath::EuclideanSpace;

        let rot_x:Basis3<f32>=Rotation3::from_angle_x(rad(self.angle.x));
        let rot_y:Basis3<f32>=Rotation3::from_angle_y(rad(self.angle.y));
        let a=rot_x.rotate_vector(vec3(0.0,0.0,self.distance));
        let b=rot_y.rotate_vector(a);

        self.camera_matrix=Matrix4::look_at(Point3::from_vec(b), self.center_position, vec3(0.0,1.0,0.0));
    }

    pub fn get_matrixes(&self) -> Option<(Matrix4<f32>, Matrix4<f32>)> {
        let perspective_matrix=match self.viewport {
            Some( ref viewport ) => viewport.perspective_matrix.clone(),
            None => return None,
        };

        Some( (perspective_matrix, self.camera_matrix.clone()) )
    }
}
