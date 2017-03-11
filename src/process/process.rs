use std;
use render;

use std::path::Path;
use std::sync::Arc;
use std::sync::RwLock;
use std::thread;
use std::thread::JoinHandle;
use std::sync::mpsc;
use std::ffi::OsString;

use ProcessError;
use Camera;
use State;
use Object;
use RenderTask;

pub enum ProcessTask{
    LoadModel(OsString),
}

pub struct Process{
    object:Arc<RwLock< Option<Object> >>,
    state:Arc<State>,
    to_render_tx:mpsc::Sender<RenderTask>,
}

impl Process{
    pub fn run(
        object:Arc<RwLock< Option<Object> >>,
        state:Arc<State>,
        to_process_rx:mpsc::Receiver<ProcessTask>,
        to_render_tx:mpsc::Sender<RenderTask>
    ) -> JoinHandle<()> {
        let process_handle=thread::spawn(move||{
            match Self::thread_function(
                object,
                state.clone(),
                to_process_rx,
                to_render_tx,
            ) {
                Ok ( _ ) => {},
                Err( e ) => {
                    use std::io::Write;
                    writeln!(&mut std::io::stderr(), "Process Error: {}!", e);
                }
            }

            state.exit();
        });

        process_handle
    }

    fn thread_function(
        object:Arc<RwLock< Option<Object> >>,
        state:Arc<State>,
        to_process_rx:mpsc::Receiver<ProcessTask>,
        to_render_tx:mpsc::Sender<RenderTask>
    ) -> Result<(),ProcessError> {
        let mut process=Process{
            object:object,
            state:state,
            to_render_tx:to_render_tx,
        };

        let loop_result=process.process_loop(&to_process_rx);

        process.state.exit();

        loop_result
    }

    fn process_loop(&mut self,to_process_rx:&mpsc::Receiver<ProcessTask>) -> Result<(),ProcessError> {
        while !self.state.should_exit() {
            match to_process_rx.recv(){
                Ok ( task ) => {
                    match task{
                        ProcessTask::LoadModel(ref file_name) => {
                            let file_name=Path::new(file_name);

                            let mut object=self.object.write().unwrap();

                            if (*object).is_none() {
                                *object=Some(Object::empty(true));
                            }

                            match *object {
                                None => {},
                                Some( ref object ) => {
                                    match object.include_collada_model(file_name, &self.to_render_tx) {
                                        Ok ( _ ) => {},
                                        Err( e ) => {println!("{}",e);},//process_data.to_render_tx.send(
                                    }
                                },
                            }
                        },//data.storage.load_lod
                    }
                },
                Err( _ ) => return Ok(()),
            }
        }

        Ok(())
    }
}
