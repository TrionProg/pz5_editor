use std;
use render;
use process;

use std::path::Path;
use std::sync::Arc;
use std::sync::{Mutex,RwLock};
use std::sync::Barrier;
use std::sync::mpsc::channel;

use std::thread;
use std::thread::JoinHandle;


use Error;
use Window;
use GUI;
use Storage;
use Camera;
use Object;
use State;


pub struct Application{}

impl Application{
    pub fn run() -> Result<(),Error> {
        let mut window=Window::new(1024, 768)?;
        let storage=Storage::new(&window)?;
        let gui=GUI::new(&window)?;

        let camera=Arc::new(RwLock::new(Camera::new(&window)?));
        let state=Arc::new(State::new());
        let object=Arc::new(RwLock::new(None));

        let (to_render_tx, to_render_rx) = channel();
        let (to_process_tx, to_process_rx) = channel();

        let process_handle=Self::run_process_thread(process::ProcessData{
            camera:camera.clone(),
            state:state.clone(),
            object:object.clone(),
            to_process_rx:to_process_rx,
            to_render_tx:to_render_tx,
        });

        Self::run_render_thread(render::RenderData{
            window:window,
            storage:storage,
            camera:camera,
            state:state,
            gui:gui,
            object:object,
            to_process_tx:to_process_tx,
            to_render_rx:to_render_rx,
        });

        process_handle.join().unwrap();

        Ok(())
    }

    fn run_process_thread(process_data:process::ProcessData) -> JoinHandle<()>{
        let process_handle=thread::spawn(move||{
            let state=process_data.state.clone();

            process::process_thread(process_data);
            state.exit();
        });

        process_handle
    }

    fn run_render_thread(render_data:render::RenderData) {
        let state=render_data.state.clone();

        render::render_thread(render_data);
        state.exit();
    }

    /*
    pub fn include_collada_model(&mut self, file_name:&Path) -> Result<(),Error> {
        if self.object.is_none() {
            self.object=Some( Object::empty(self.render.clone()) );
        }

        match self.object {
            Some( ref mut object ) => {object.include_collada_model(file_name)?;},
            None => {},
        }

        Ok(())
    }
    */
}
