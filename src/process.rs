use std;
use render;

use std::sync::Arc;
use std::sync::RwLock;
use std::thread;
use std::sync::mpsc;
use std::ffi::OsString;

use Camera;
use State;
use render::RenderTask;


pub struct ProcessData{
    pub camera:Arc<RwLock<Camera>>,
    pub state:Arc<State>,
    pub to_process_rx:mpsc::Receiver<ProcessTask>,
    pub to_render_tx:mpsc::Sender<RenderTask>,
}

pub enum ProcessTask{
    LoadModel(OsString),
}


pub fn process_thread(process_data:ProcessData){
    let mut data=process_data;

    let mut i=0;
    while !data.state.should_exit() {
        thread::sleep_ms(100);

        i+=1;

        if i==10 {
            break;
        }
    }
}
