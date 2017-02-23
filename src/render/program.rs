use std;
use glium;

use std::rc::Rc;
use std::collections::HashMap;

use Error;

#[derive(Copy,Clone)]
enum Dimension{
    V2D,
    V3D,
}

pub struct Program{
    glium_program:glium::Program,
}

impl Program{
    fn generate_program(display:&glium::backend::glutin_backend::GlutinFacade, dimension:Dimension, is_normal:bool, is_tex_coords:bool) -> Result<(String,Program),Error>{
        let full_vertex_format=Self::generate_full_vertex_format(dimension,is_normal,is_tex_coords);
        let vertex_code=Self::generate_vertex_code(dimension,is_normal,is_tex_coords);
        let fragment_code=Self::generate_fragment_code(dimension,is_normal,is_tex_coords);

        let glium_program=glium::Program::from_source(display,vertex_code.as_str(),fragment_code.as_str(),None)?;

        Ok(
            (
                full_vertex_format,
                Program{
                    glium_program:glium_program,
                }
            )
        )
    }

    fn generate_full_vertex_format(dimension:Dimension, is_normal:bool, is_tex_coords:bool) -> String{
        let mut full_vertex_format=String::with_capacity(32);

        full_vertex_format.push_str(
            match dimension {
                Dimension::V2D => "VERTEX(X:f32,Y:f32)",
                Dimension::V3D => "VERTEX(X:f32,Y:f32,Z:f32)",
            }
        );

        if is_normal {
            full_vertex_format.push_str(
                match dimension {
                    Dimension::V2D => " NORMAL(X:f32,Y:f32)",
                    Dimension::V3D => " NORMAL(X:f32,Y:f32,Z:f32)",
                }
            );
        }

        if is_tex_coords {
            full_vertex_format.push_str( " TEXCOORD(U:f32,V:f32)" );
        }

        full_vertex_format
    }

    fn generate_vertex_code(dimension:Dimension, is_normal:bool, is_tex_coords:bool) -> String{
        let mut vertex_code=String::with_capacity(256);

        vertex_code.push_str(
            "
#version 140
uniform mat4 persp_matrix;
uniform mat4 view_matrix;"
        );

        // IN

        vertex_code.push_str(
            match dimension{
                Dimension::V2D => "
in vec2 position;",
                Dimension::V3D => "
in vec3 position;",
            }
        );

        if is_normal {
            vertex_code.push_str(
                match dimension{
                    Dimension::V2D => "
in vec2 normal;",
                    Dimension::V3D => "
in vec3 normal;"
                }
            );
        }

        if is_tex_coords {
            vertex_code.push_str("
in vec2 tex_coords;"
            );
        }

        // OUT

        vertex_code.push_str("
out vec3 v_position;",
        );

        if is_normal {
            vertex_code.push_str("
out vec3 v_normal;"
            );
        }

        if is_tex_coords {
            vertex_code.push_str("
out vec2 v_tex_coords;"
            );
        }

        // MAIN

        vertex_code.push_str("\nvoid main(){");

        vertex_code.push_str(
            match dimension{
                Dimension::V2D => "
    v_position=vec3(position,0.0);",
                Dimension::V3D => "
    v_position=position;",
            }
        );

        if is_normal {
            vertex_code.push_str(
                match dimension{
                    Dimension::V2D => "
    v_normal=vec3(normal,0.0);",
                    Dimension::V3D => "
    v_normal=normal;",
                }
            );
        }

        if is_tex_coords {
            vertex_code.push_str("
    v_tex_coords=tex_coords;"
            );
        }

        vertex_code.push_str("
    gl_Position = persp_matrix * view_matrix * vec4(v_position, 1.0);"
        );

        vertex_code.push_str("\n}");

        vertex_code
    }

    fn generate_fragment_code(dimension:Dimension, is_normal:bool, is_tex_coords:bool) -> String{
        let mut fragment_code=String::with_capacity(256);

        fragment_code.push_str(
            "
#version 140"
        );

        // IN

        fragment_code.push_str("
in vec3 v_position;"
        );

        if is_normal {
            fragment_code.push_str("
in vec3 v_normal;"
            );
        }

        if is_tex_coords {
            fragment_code.push_str("
in vec2 tex_coords;"
            );
        }

        fragment_code.push_str("
out vec4 f_color;
const vec3 LIGHT = vec3(-0.2, 0.8, 0.1);
void main() {"
        );

        fragment_code.push_str(
            if is_normal {
                "
    float lum = max(dot(normalize(v_normal), normalize(LIGHT)), 0.0);"
            }else{
                "
    float lum = 1.0;"
            }
        );

        fragment_code.push_str("
    vec3 color = (0.3 + 0.7 * lum) * vec3(1.0, 1.0, 1.0);
    f_color = vec4(color, 1.0);");

        fragment_code.push_str("\n}");

        fragment_code
    }



    pub fn generate_programs(display:&glium::backend::glutin_backend::GlutinFacade) -> Result< HashMap<String,Rc<Program>> ,Error>{
        let mut programs=HashMap::new();

        for dimension in [Dimension::V2D,Dimension::V3D].into_iter(){
            for is_normal in [false,true].into_iter(){
                for is_tex_coords in [false,true].into_iter(){
                    let (full_vertex_format, program)=Program::generate_program(display, *dimension, *is_normal, *is_tex_coords)?;

                    programs.insert(full_vertex_format, Rc::new(program) );
                }
            }
        }

        Ok( programs )
    }
}
