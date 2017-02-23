
use std::path::Path;
use std::rc::Rc;

use Error;
use Object;
use Render;

pub struct Application{
    gui:bool,
    object:Option<Object>,
    render:Option<Rc<Render>>,
}

impl Application{
    pub fn new(gui:bool) -> Result<Application, Error>{
        let render=if gui {
            Some(Rc::new( Render::new()? ))
        }else{
            None
        };

        Ok(
            Application{
                gui:gui,
                object:None,
                render:render,
            }
        )
    }

    pub fn include_collada_model(&mut self, file_name:&Path) -> Result<(),Error> {
        if self.object.is_none() {
            self.object=Some( Object::empty(self.render.clone()) );
        }

        match self.object {
            Some( ref mut object ) => {object.include_collada_model(file_name)?;},
            None => {},
        }

        Ok(())
    }
}
