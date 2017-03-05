use std;
use render;

use std::path::Path;
use std::sync::Arc;
use std::sync::RwLock;
use std::thread;
use std::sync::mpsc;
use std::ffi::OsString;

use Camera;
use State;
use Object;
use render::RenderTask;


pub struct ProcessData{
    pub camera:Arc<RwLock<Camera>>,
    pub state:Arc<State>,
    pub object:Arc<RwLock< Option<Object> >>,
    pub to_process_rx:mpsc::Receiver<ProcessTask>,
    pub to_render_tx:mpsc::Sender<RenderTask>,
}

pub enum ProcessTask{
    LoadModel(OsString),
}


pub fn process_thread(process_data:ProcessData){
    let mut data=process_data;

    while !data.state.should_exit() {
        match data.to_process_rx.recv(){
            Ok ( task ) => {
                match task{
                    ProcessTask::LoadModel(ref file_name) => {
                        let file_name=Path::new(file_name);

                        let mut object=data.object.write().unwrap();

                        if (*object).is_none() {
                            *object=Some(Object::empty(true));
                        }

                        match *object {
                            None => {},
                            Some( ref object ) => {
                                match object.include_collada_model(file_name) {
                                    Ok ( _ ) => {},
                                    Err( e ) => {println!("{}",e);},//process_data.to_render_tx.send(
                                }
                            },
                        }
                    },//data.storage.load_lod
                }
            },
            Err( _ ) => return,
        }
    }
}
