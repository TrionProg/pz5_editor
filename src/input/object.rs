use std;
use pz5;
use pz5_collada;

use std::rc::Rc;

use std::collections::HashMap;

use super::Model;

pub struct Object{
    pub models:Vec<Rc<Model>>,
}

impl Object{
    pub fn empty() -> Self{
        Object{
            models:Vec::new(),
        }
    }
}
