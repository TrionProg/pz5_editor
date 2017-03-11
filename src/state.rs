use std;

use std::sync::Mutex;

pub struct State{
    exit:Mutex<bool>,
}

impl State{
    pub fn new() -> Self{
        let state=State{
            exit:Mutex::new(false),
        };

        state
    }

    pub fn should_exit(&self) -> bool{
        *self.exit.lock().unwrap()
    }

    pub fn exit(&self){
        *self.exit.lock().unwrap()=true;
    }
}
