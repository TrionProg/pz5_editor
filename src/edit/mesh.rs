use std;
use pz5;
use pz5_collada;
use input;

use std::rc::Rc;

use super::LOD;

pub struct Mesh{
    input_mesh:Rc<input::Mesh>,
    name:String,
    vertex_full_format:String,
    lods:Vec<LOD>,
    description:String,
}

impl Mesh{
    pub fn new(input_mesh:Rc<input::Mesh>) -> Self{
        let name=input_mesh.name.clone();
        let vertex_full_format=input_mesh.vertex_full_format.clone();

        let mut lods=Vec::with_capacity(2);
        for input_lod in input_mesh.lods.iter(){
            let lod=LOD::new(input_lod.clone());

            lods.push(lod);
        }

        Mesh{
            input_mesh:input_mesh,
            name:name,
            vertex_full_format:vertex_full_format,
            lods:lods,
            description:String::new(),
        }
    }
}
