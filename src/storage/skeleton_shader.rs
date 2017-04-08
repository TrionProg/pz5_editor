use std;
use glium;

use RenderError;
use Window;


pub struct SkeletonShader{
    pub glium_program:glium::Program,
}

impl SkeletonShader{
    pub fn new(window:&Window) -> Result<Self,RenderError> {
        let vertex_code = "
            #version 140

            uniform mat4 perspective_matrix;
            uniform mat4 camera_matrix;


            layout (std140) uniform BonesArray {
                mat4 bone_matrices[256];
            };


            //in vec3 position;
            in uint bone_index;
            in float color;
            out float v_color;
            void main() {
                v_color=color;
                gl_Position = perspective_matrix * camera_matrix * bone_matrices[bone_index]/2 * vec4(0.0,0.0,0.0,1.0);
            }
        ";

        let fragment_code = "
            #version 140
            in float v_color;
            out vec4 f_color;
            void main() {
                f_color = vec4(v_color, v_color, 0.0, 1.0);
            }
        ";


        let glium_program=glium::Program::from_source(&window.display,vertex_code,fragment_code,None)?;

        let shader=SkeletonShader{
            glium_program:glium_program,
        };

        Ok( shader )
    }
}
