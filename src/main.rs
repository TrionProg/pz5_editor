extern crate pz5;
extern crate collada;
#[macro_use]
extern crate glium;
extern crate glutin;
extern crate cgmath;
extern crate object_pool;
extern crate byteorder;

use std::env;
use std::io::Write;

//pub mod id;
//pub use id::ID;

//pub mod growable_slab;
//pub use growable_slab::{GrowableSlab,SlabElement};
/*
pub mod error;
pub use error::Error;
*/

pub mod window;
pub use window::Window;

pub mod gui;
pub use gui::GUI;

pub mod storage;
pub use storage::Storage;

pub mod camera;
pub use camera::Camera;

pub mod object;
pub use object::Object;

pub mod from_collada;

pub mod state;
pub use state::State;

pub mod render;
pub use render::{Render,RenderFrame,RenderError,RenderTask,RenderSender};

pub mod process;
pub use process::{Process,ProcessError,ProcessTask,ProcessSender};

pub mod application;
pub use application::Application;

fn main(){
    let mut args=env::args();
    let gui_mode=match args.nth(0) {
        Some( ref mode ) => {
            match mode.as_str() {
                "convert" => false,
                _ => true,
            }
        },
        None => true,
    };

    if gui_mode{
        Application::run();
    }
}

/*
use std::path::Path;
use pz5::ToPz5Model;

mod model;
pub use model::Model;

mod mesh;
pub use mesh::Mesh;

mod lod;
pub use lod::LOD;

#[macro_use]
extern crate glium;

mod support;

use glium::Surface;
use glium::glutin;
use glium::index::PrimitiveType;

use pz5::ToPz5LOD;

fn main() {
    let model=match Model::load_from_collada(Path::new("pz5.dae")){
        Ok( m ) => m,
        Err(e ) => {println!("Error: {}",e); return; }
    };

    model.print();

    {
        let mut file=std::fs::File::create("model.pz5").unwrap();
        match model.write_pz5(&mut file){
            Ok ( _ ) => {},
            Err( e ) => {println!("Error: {}",e); return; }
        }
    }

    //return;

    use glium::DisplayBuild;

    // building the display, ie. the main object
    let display = glutin::WindowBuilder::new()
        .with_depth_buffer(24)
        .build_glium()
        .unwrap();

    // building the vertex buffer, which contains all the vertices that we will draw
    let vertex_buffer = {
        #[derive(Copy, Clone)]
        struct Vertex {
            position: [f32; 3],
            normal: [f32; 3],
        }

        implement_vertex!(Vertex, position, normal);

        use std::mem::transmute;
        //let bytes: [u8; 4] = unsafe { transmute(123u32.to_be()) };

        let lod=&model.meshes.get("Gun").unwrap().lods[0];

        let vertices=unsafe{
            let v=std::mem::transmute::<&[u8], &[Vertex]>(lod.get_all_data());
            //std::mem::transmute::<&[u8], &[Vertex]>( model.meshes.get("Cube").unwrap().lods[0].get_all_data() )
            std::slice::from_raw_parts::<Vertex>( &v[0] as *const Vertex , lod.vertices_count )
        };

        println!("VN{}",vertices.len());

        glium::VertexBuffer::new(&display,
            /*
            &[
                Vertex { position: [-0.5, -0.5, 0.0], normal: [0.0, 1.0, 0.0] },
                Vertex { position: [ 0.0,  0.5, 0.0], normal: [0.0, 0.0, 1.0] },
                Vertex { position: [ 0.5, -0.5, 0.0], normal: [1.0, 0.0, 0.0] },
            ]
            */

            vertices
        ).unwrap()
    };

    // building the index buffer
    //let index_buffer = glium::IndexBuffer::new(&display, PrimitiveType::TrianglesList,
    //                                           &[0u16, 1, 2]).unwrap();

    // compiling shaders and linking them together
    let program = program!(&display,
        140 => {
            vertex: "
                #version 140
                uniform mat4 persp_matrix;
                uniform mat4 view_matrix;
                in vec3 position;
                in vec3 normal;
                out vec3 v_position;
                out vec3 v_normal;
                void main() {
                    v_position = position;
                    v_normal = normal;
                    gl_Position = persp_matrix * view_matrix * vec4(v_position * 0.005, 1.0);
                }
            ",

            fragment: "
                #version 140
                in vec3 v_normal;
                out vec4 f_color;
                const vec3 LIGHT = vec3(-0.2, 0.8, 0.1);
                void main() {
                    float lum = max(dot(normalize(v_normal), normalize(LIGHT)), 0.0);
                    vec3 color = (0.3 + 0.7 * lum) * vec3(1.0, 1.0, 1.0);
                    f_color = vec4(color, 1.0);
                }
            ",
        }
    ).unwrap();

    let mut camera = support::camera::CameraState::new();

    // the main loop
    support::start_loop(|| {
        camera.update();

        // building the uniforms
        let uniforms = uniform! {
            persp_matrix: camera.get_perspective(),
            view_matrix: camera.get_view(),
        };

        // draw parameters
        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::DepthTest::IfLess,
                write: true,
                .. Default::default()
            },
            .. Default::default()
        };

        // drawing a frame
        let mut target = display.draw();
        target.clear_color_and_depth((0.0, 0.0, 0.0, 0.0), 1.0);

        target.draw(&vertex_buffer,
                    &glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
                    &program, &uniforms, &params).unwrap();
        target.finish().unwrap();

        //target.draw(&vertex_buffer, &index_buffer, &program, &uniforms, &Default::default()).unwrap();
        //target.finish().unwrap();

        // polling and handling the events received by the window
        for event in display.poll_events() {
            match event {
                glutin::Event::Closed => return support::Action::Stop,
                ev => camera.process_input(&ev),
            }
        }

        // polling and handling the events received by the window
        for event in display.poll_events() {
            match event {
                glutin::Event::Closed => return support::Action::Stop,
                _ => ()
            }
        }

        support::Action::Continue
    });
}
*/
