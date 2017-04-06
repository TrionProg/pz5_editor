use std;
use pz5;
use process;
use object;
use glutin;
use glium;
use object_pool;

use std::sync::Arc;
use std::sync::RwLock;
use std::thread;
use std::sync::mpsc;
use std::time::{Duration, Instant};

use RenderError;
use Window;
use Storage;
use Camera;
use State;
use GUI;
use Object;
use process::ProcessTask;
use super::RenderFrame;

use glutin::ElementState;
use glutin::Event as WindowEvent;

use pz5::GeometryType;
use pz5::Pz5Geometry;

use object_pool::growable::ID;

pub enum RenderTask{
    Error(String),
    LoadLOD(Arc<object::LOD>, Pz5Geometry, String, GeometryType),
    RemoveLOD(Arc<object::LOD>,ID),
}

pub struct Render{
    window:Window,
    gui:GUI,
    storage:Storage,
    camera:Camera,
    object:Arc<RwLock< Option<Object> >>,
    state:Arc<State>,
    to_process_tx:mpsc::Sender<ProcessTask>,
}

impl Render {
    pub fn run(
        object:Arc<RwLock< Option<Object> >>,
        state:Arc<State>,
        to_process_tx:mpsc::Sender<ProcessTask>,
        to_render_rx:mpsc::Receiver<RenderTask>
    ) {
        match Self::thread_function(
            object,
            state.clone(),
            to_process_tx,
            to_render_rx,
        ) {
            Ok ( _ ) => {},
            Err( e ) => {
                use std::io::Write;
                writeln!(&mut std::io::stderr(), "Render Error: {}!", e);
            }
        }

        state.exit();
    }

    fn thread_function(
        object:Arc<RwLock< Option<Object> >>,
        state:Arc<State>,
        to_process_tx:mpsc::Sender<ProcessTask>,
        to_render_rx:mpsc::Receiver<RenderTask>
    ) -> Result<(),RenderError> {
        let mut window=Window::new(1024, 768)?;
        let mut storage=Storage::new(&window)?;
        let mut gui=GUI::new(&window)?;
        let mut camera=Camera::new(&window)?;

        let mut render=Render{
            window:window,
            gui:gui,
            storage:storage,
            camera:camera,
            object:object,
            state:state,
            to_process_tx:to_process_tx,
        };

        let loop_result=render.render_loop(&to_render_rx);

        render.state.exit();

        //clear

        loop_result
    }

    fn render_loop(&mut self, to_render_rx:&mpsc::Receiver<RenderTask>) -> Result<(),RenderError>{
        let vertex_buffer = {
            #[derive(Copy, Clone)]
            struct Vertex {
                position: [f32; 3],
                normal: [f32; 3],
            }

            implement_vertex!(Vertex, position, normal);

            use std::mem::transmute;

            glium::VertexBuffer::new(&self.window.display,
                &[
                    Vertex { position: [-0.5, -0.5, 0.0], normal: [0.0, 1.0, 0.0] },
                    Vertex { position: [ 0.0,  0.5, 0.0], normal: [0.0, 0.0, 1.0] },
                    Vertex { position: [ 0.5, -0.5, 0.0], normal: [1.0, 0.0, 0.0] },
                ]
            ).unwrap()
        };

        // building the index buffer
        //let index_buffer = glium::IndexBuffer::new(&display, PrimitiveType::TrianglesList,
        //                                           &[0u16, 1, 2]).unwrap();

        // compiling shaders and linking them together
        let program = program!(&self.window.display,
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
                        gl_Position = persp_matrix * view_matrix * vec4(v_position, 1.0);
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

        let mut next_frame_time=Instant::now();

        while !self.state.should_exit() {
            loop{
                match to_render_rx.try_recv(){
                    Ok ( task ) => {
                        match task{
                            RenderTask::Error(_) => {},
                            RenderTask::LoadLOD(lod,geometry,vertex_format,geometry_type) =>
                                self.storage.load_geometry(lod, geometry, geometry_type, vertex_format, &self.window)?,
                            RenderTask::RemoveLOD(lod,geometry_id) => {},
                        }
                    },
                    Err( mpsc::TryRecvError::Empty ) => break,
                    Err( mpsc::TryRecvError::Disconnected ) => return Ok(()),
                }
            }

            while let Some(event)=self.window.display.poll_events().next() {
                match event {
                    WindowEvent::Closed => return Ok(()),
                    WindowEvent::Resized(width, height) =>
                        self.on_window_resize(width,height),
                    WindowEvent::MouseMoved(x,y) =>
                        self.on_mouse_move(x,y),
                    WindowEvent::MouseInput(state,mouse_button) =>
                        self.on_mouse_button(state,mouse_button),
                    WindowEvent::MouseWheel(delta,_) =>
                        self.on_mouse_wheel(delta),
                    WindowEvent::DroppedFile(path) =>
                        {self.to_process_tx.send( ProcessTask::LoadModel(path.into_os_string()) );},//TODO:process error(return Err)
                    _ => {},
                }
            }

            if Instant::now()>next_frame_time {
                self.render()?;
                /*
                match RenderFrame::new(&self.camera, &self.window, &self.storage) {
                    Some( mut frame ) => {
                        self.storage.grid.render(&mut frame, &self.storage.grid_shader)?;
                        use glium::Surface;
                        /*
                        let uniforms = uniform! {
                            perspective_matrix: Into::<[[f32; 4]; 4]>::into(frame.perspective_matrix),
                            camera_matrix: Into::<[[f32; 4]; 4]>::into(frame.camera_matrix),
                        };

                        frame.target.draw(&self.storage.grid.vbo,
                            &glium::index::NoIndices(glium::index::PrimitiveType::LinesList),
                            &self.storage.grid_shader.glium_program, &uniforms, &frame.draw_parameters).unwrap();
                        */
                        let uniforms = uniform! {
                            //persp_matrix:Into::<[[f32; 4]; 4]>::into(perspective_matrix),
                            persp_matrix: Into::<[[f32; 4]; 4]>::into(frame.perspective_matrix),
                            view_matrix: Into::<[[f32; 4]; 4]>::into(frame.camera_matrix),
                            //persp_matrix:
                            //persp_matrix: camera.get_perspective(),
                            //view_matrix: Into::<[[f32; 4]; 4]>::into(view_matrix),//camera.get_view(),
                            //view_matrix: Into::<[[f32; 4]; 4]>::into(frame.camera_matrix),
                        };

                        frame.target.draw(&vertex_buffer,
                                    &glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
                                    &program, &uniforms, &frame.draw_parameters).unwrap();

                        frame.finish();
                    },
                    None => {},
                }
                //self.render();
                */

                next_frame_time+=Duration::from_millis(20);
            }else{
                thread::sleep_ms(1);
            }
        }

        Ok(())
    }

    fn on_window_resize(&mut self,width:u32,height:u32) {
        self.window.resize(width, height);
        self.camera.resize(&self.window);
    }

    fn on_mouse_move(&mut self, x:i32, y:i32) {
        self.gui.on_mouse_move(x,y);

        if self.gui.input.left_mouse_button==ElementState::Pressed {
            self.camera.rotate(&self.gui.input);
        }
    }

    fn on_mouse_button(&mut self, state:ElementState, mouse_button:glutin::MouseButton) {
        self.gui.on_mouse_button(state, mouse_button);

        if self.gui.input.right_mouse_button==ElementState::Pressed {
            use std::ffi::OsString;
            let mut file_name=OsString::new();
            file_name.push("pz5.dae");
            //file_name.push("box.dae");
            //file_name.push("anim2.dae");
            self.to_process_tx.send( ProcessTask::LoadModel(file_name) );
        }
    }

    fn on_mouse_wheel(&mut self,delta:glutin::MouseScrollDelta) {
        self.camera.on_mouse_wheel(&mut self.storage, &self.window, delta);
    }

    fn render(&mut self) -> Result<(),RenderError> {
        match RenderFrame::new(&self.camera, &self.window, &self.storage) {
            Some( mut frame ) => {
                self.storage.grid.render(&mut frame, &self.storage.grid_shader)?;

                match *self.object.read().unwrap() {
                    Some( ref object ) =>
                        object.render( &mut frame )?,
                    None => {},
                }

                frame.finish();
            },
            None => {},
        }

        Ok(())
    }
}


/*
use std;
use glium;
use glutin;

use std::rc::Rc;
use std::collections::HashMap;

use Error;
use super::Program;

use ObjectFrame;

use support;
use support::camera::CameraState;

const STARTUP_WINDOW_WIDTH:u32=1024;
const STARTUP_WINDOW_HEIGHT:u32=768;

pub struct Render{
    pub display:glium::backend::glutin_backend::GlutinFacade,
    pub programs:HashMap<String,Rc<Program>>,
    pub window_width:u32,
    pub window_height:u32,
}

impl Render{
    pub fn new() -> Result<Self,Error>{
        use glium::DisplayBuild;

        let display=glutin::WindowBuilder::new()
            .with_title(String::from("pz5 editor"))
            .with_dimensions(STARTUP_WINDOW_WIDTH,STARTUP_WINDOW_HEIGHT)
            .with_gl(glutin::GlRequest::Latest)
            .with_depth_buffer(24)
            .build_glium()?;

        let programs=Program::generate_programs(&display)?;

        Ok(
            Render{
                display:display,
                programs:programs,
                window_width:STARTUP_WINDOW_WIDTH,
                window_height:STARTUP_WINDOW_HEIGHT,
            }
        )
    }
}
*/
