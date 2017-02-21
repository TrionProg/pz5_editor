use std;
use pz5;
use pz5_collada;
use input;

use std::rc::Rc;

pub struct LOD{
    input_lod:Rc<input::LOD>,
    distance:f32,
    description:String,
}

impl LOD{
    pub fn new(input_lod:Rc<input::LOD>) -> Self{
        let distance=input_lod.distance;

        LOD{
            input_lod:input_lod,
            distance:distance,
            description:String::new(),
        }
    }
}
