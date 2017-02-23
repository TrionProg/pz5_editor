use std;
use pz5;
use pz5_collada;
use input;

use std::rc::Rc;

use super::Model;

pub struct Object{
    pub models:Vec<Model>,
}

impl Object{
    pub fn empty() -> Self{
        Object{
            models:Vec::new(),
        }
    }
}
