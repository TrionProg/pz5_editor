

use std::rc::Rc;
use std::cell::RefCell;

use location::Location;
use super::VirtualModel;

pub struct VirtualInstance<'a>{
    pub name:String,
    pub location:Location,
    pub model:Rc<RefCell< VirtualModel<'a> >>,
}

impl<'a> VirtualInstance<'a> {
    pub fn new(model:&Rc<RefCell< VirtualModel<'a> >>, name:String, location:Location) -> Self {
        VirtualInstance{
            name:name,
            location:location,
            model:model.clone(),
        }
    }
}
