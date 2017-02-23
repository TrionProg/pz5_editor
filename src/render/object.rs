
use super::Model;

pub struct Object{
    models:Vec<Model>,
}

impl Object{
    pub fn empty() -> Self{
        Object{
            models:Vec::new(),
        }
    }
}
