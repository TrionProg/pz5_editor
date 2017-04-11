use std;
use pz5;
use process;
use object;
use storage;
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
    LoadSkeleton(Arc<object::Model>, Vec<storage::skeleton::Vertex>, Vec<storage::skeleton::Vertex>),
    RemoveSkeleton(Arc<object::Model>,ID),
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
        let mut next_frame_time=Instant::now();

        while !self.state.should_exit() {
            loop{
                match to_render_rx.try_recv(){
                    Ok ( task ) => {
                        match task{
                            RenderTask::Error(_) => {},
                            RenderTask::LoadLOD(lod,geometry,vertex_format,geometry_type) =>
                                self.storage.load_geometry(lod, geometry, geometry_type, vertex_format, &self.window)?,
                            RenderTask::RemoveLOD(lod,geometry_id) => {},//TODO:removeLOD
                            RenderTask::LoadSkeleton(skeleton,joints_geometry, bones_geometry) =>
                                self.storage.load_skeleton(skeleton, joints_geometry, bones_geometry, &self.window)?,
                            RenderTask::RemoveSkeleton(model,skeleton_id) => {},//TODO:removeSkeleton
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
            //file_name.push("pz5.dae");
            //file_name.push("box.dae");
            file_name.push("scene.dae");
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
                    Some( ref object ) => {
                        object.render( &mut frame )?;
                        frame.skeleton_mode();
                        object.render_skeletons( &mut frame )?;
                    },
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
