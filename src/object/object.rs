use std;


use std::path::Path;
use std::rc::Rc;
use std::collections::HashMap;
use std::collections::hash_map::Entry;

use Error;
use Render;

use super::Model;

pub struct Object{
    model_id:usize,
    pub models:HashMap<usize, Rc<Model>>,
    render:Option<Rc<Render>>,
}

impl Object{
    pub fn empty(render:Option<Rc<Render>>) -> Self{
        Object{
            model_id:1,
            models:HashMap::new(),
            render:render,
        }
    }

    pub fn include_collada_model(&mut self, file_name:&Path) -> Result<(),Error> {
        let model=Model::include_collada_model(file_name, self.model_id, &self.render)?;

        self.models.insert(self.model_id, Rc::new(model));
        self.model_id+=1;

        Ok(())
    }
}
