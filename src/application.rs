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
use Process;
use Render;


pub struct Application{}

impl Application{
    pub fn run() -> Result<(),Error> {
        let state=Arc::new(State::new());
        let object=Arc::new(RwLock::new(None));

        let (to_render_tx, to_render_rx) = channel();
        let (to_process_tx, to_process_rx) = channel();

        let process_handle=Process::run(
            object.clone(),
            state.clone(),
            to_process_rx,
            to_render_tx,
        );

        Render::run(
            object,
            state,
            to_process_tx,
            to_render_rx
        );

        process_handle.join().unwrap();

        Ok(())
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
