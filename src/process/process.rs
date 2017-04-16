use std;
use render;

use std::path::Path;
use std::sync::Arc;
use std::sync::RwLock;
use std::thread;
use std::thread::JoinHandle;
use std::sync::mpsc;
use std::ffi::OsString;

use super::Error;

use super::Storage;
use Camera;
use State;

pub enum Task{
    LoadModel(OsString),
}

pub struct Process{
    process_storage:Arc< Storage >,
    state:Arc<State>,
    to_render_tx:mpsc::Sender<render::Task>,
}

impl Process{
    pub fn run(
        process_storage:Arc< Storage >,
        state:Arc<State>,
        to_process_rx:mpsc::Receiver<Task>,
        to_render_tx:mpsc::Sender<render::Task>
    ) -> JoinHandle<()> {
        let process_handle=thread::spawn(move||{
            match Self::thread_function(
                process_storage,
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
        process_storage:Arc< Storage >,
        state:Arc<State>,
        to_process_rx:mpsc::Receiver<Task>,
        to_render_tx:mpsc::Sender<render::Task>
    ) -> Result<(),Error> {
        let mut process=Process{
            process_storage:process_storage,
            state:state,
            to_render_tx:to_render_tx,
        };

        let loop_result=process.process_loop(&to_process_rx);

        process.state.exit();

        loop_result
    }

    fn process_loop(&mut self,to_process_rx:&mpsc::Receiver<Task>) -> Result<(),Error> {
        while !self.state.should_exit() {
            match to_process_rx.recv(){
                Ok ( task ) => {
                    match task{
                        Task::LoadModel(ref file_name) => {
                            let file_name=Path::new(file_name);

                            match self.process_storage.include_collada_model(file_name, &self.to_render_tx) {
                                Ok ( _ ) => {},
                                Err( e ) => {println!("{}",e);},//process_data.to_render_tx.send(
                            }
                        },
                    }
                },
                Err( _ ) => return Ok(()),
            }
        }

        Ok(())
    }
}
