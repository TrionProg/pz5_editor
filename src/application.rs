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

use State;

pub struct Application{}

impl Application{
    pub fn run(gui_mode:bool) {
        let state=Arc::new( State::new() );
        let process_storage=Arc::new( process::Storage::empty(gui_mode) );

        let (to_render_tx, to_render_rx) = channel();
        let (to_process_tx, to_process_rx) = channel();

        let process_handle=process::Process::run(
            process_storage.clone(),
            state.clone(),
            to_process_rx,
            to_render_tx,
        );

        render::Render::run(
            process_storage,
            state,
            to_process_tx,
            to_render_rx
        );

        process_handle.join().unwrap();
    }
}
