use std;
use pz5;
use process;

use std::sync::Arc;
use std::sync::RwLock;
use std::thread;
use std::thread::JoinHandle;
use std::sync::mpsc;

use Error;
use RenderError;
use Window;
use Storage;
use Camera;
use State;
use GUI;
use Object;
use process::ProcessTask;

use pz5::GeometryType;
use pz5::Pz5Geometry;

pub struct RenderData{
    pub window:Window,
    pub storage:Storage,
    pub camera:Arc<RwLock<Camera>>,
    pub state:Arc<State>,
    pub gui:GUI,
    pub object:Arc<RwLock< Option<Object> >>,
    pub to_process_tx:mpsc::Sender<ProcessTask>,
    pub to_render_rx:mpsc::Receiver<RenderTask>,
}

pub enum RenderTask{
    Error(String),
    LoadLOD{geometry:Pz5Geometry, vertex_format:String, geometry_type:GeometryType},
}

pub fn render_thread(render_data:RenderData) -> Result<(),RenderError>{
    let mut data=render_data;

    while !data.state.should_exit() {
        loop{
            match data.to_render_rx.try_recv(){
                Ok ( task ) => {
                    match task{
                        RenderTask::Error(_) => {},
                        RenderTask::LoadLOD{geometry,vertex_format,geometry_type} => {},//data.storage.load_lod
                    }
                },
                Err( mpsc::TryRecvError::Empty ) => break,
                Err( mpsc::TryRecvError::Disconnected ) => return Ok(()),
            }
        }
        thread::sleep_ms(100);
    }

    Ok(())
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
