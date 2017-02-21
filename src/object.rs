use std;

use input;
use edit;


use std::path::Path;
use std::rc::Rc;
use std::collections::hash_map::Entry;

use Error;

pub struct Object{
    input_object: input::Object,
    edit_object:  edit::Object,
}

impl Object{
    pub fn empty() -> Self{
        Object{
            input_object: input::Object::empty(),
            edit_object:  edit::Object::empty(),
        }
    }

    pub fn include_collada_model(&mut self, file_name:&Path) -> Result<(),Error> {
        let input_model=Rc::new( input::Model::load_from_collada(file_name)? );

        self.input_object.models.push(input_model.clone());
        self.edit_object.models.push( edit::Model::new(input_model) );

        Ok(())
    }
}
