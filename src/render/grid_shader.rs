use std;
use glium;

use super::Error;
use super::Window;

pub struct GridShader{
    pub glium_program:glium::Program,
}

impl GridShader{
    pub fn new(window:&Window) -> Result<Self,Error> {
        let vertex_code = "
            #version 140
            uniform mat4 perspective_matrix;
            uniform mat4 camera_matrix;
            in vec2 position;
            void main() {
                gl_Position = perspective_matrix * camera_matrix * vec4(position.x, 0.0, position.y, 1.0);
            }
        ";

        let fragment_code = "
            #version 140
            out vec4 f_color;
            void main() {
                f_color = vec4(0.8, 0.8, 0.8, 1.0);
            }
        ";


        let glium_program=glium::Program::from_source(&window.display,vertex_code,fragment_code,None)?;

        let shader=GridShader{
            glium_program:glium_program,
        };

        Ok( shader )
    }
}
