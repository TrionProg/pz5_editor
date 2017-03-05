use std;
use render;

use std::path::Path;
use std::rc::Rc;
use std::collections::HashMap;

use Error;
use Window;

pub struct Storage{
    model_shaders:HashMap<String,Rc<render::ModelShader>>,
    //textures:
}

impl Storage{
    pub fn new(window:&Window) -> Result<Self,Error> {
        let model_shaders = render::ModelShader::generate_model_shaders(window)?;

        let storage=Storage{
            model_shaders:model_shaders,
        };

        Ok(storage)
    }
}
