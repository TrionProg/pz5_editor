use std;
use render;
use object_pool;

use std::path::Path;
use std::rc::Rc;
use std::collections::HashMap;
use object_pool::growable::{Pool,ID,Slot};

use RenderError;
use Window;
use super::Grid;
use super::GridShader;
use super::ModelShader;
//use super::Geometry;

pub struct Storage{
    pub model_shaders:HashMap<String,Rc<ModelShader>>,
    pub grid_shader:GridShader,
    pub grid:Grid,
    //pub geometries:Pool<Geometry,Geometry>,
    //textures:
}

impl Storage{
    pub fn new(window:&Window) -> Result<Self,RenderError> {
        let model_shaders = ModelShader::generate_model_shaders(window)?;

        let grid_shader=GridShader::new(window)?;
        let grid=Grid::new(10.0, window)?;

        let storage=Storage{
            model_shaders:model_shaders,
            grid_shader:grid_shader,
            grid:grid,
        };

        Ok(storage)
    }
}
