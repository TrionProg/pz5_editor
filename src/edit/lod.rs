use std;
use pz5;
use pz5_collada;
use input;
use render;

use std::rc::Rc;

pub struct LOD{
    input_lod:Rc<input::LOD>,

    distance:f32,
    include:bool,
    display:bool,

    description:String,
    render_lod:Option<render::LOD>,
}

impl LOD{
    pub fn new(input_lod:Rc<input::LOD>) -> Self{
        let distance=input_lod.distance;

        LOD{
            input_lod:input_lod,

            distance:distance,
            include:true,
            display:true,

            description:String::new(),
            render_lod:None,
        }
    }
}
