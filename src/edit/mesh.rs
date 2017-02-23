use std;
use pz5;
use pz5_collada;
use input;

use std::rc::Rc;

use super::LOD;

pub struct Mesh{
    input_mesh:Rc<input::Mesh>,

    name:String,
    full_vertex_format:String,
    include:bool,
    display:bool,

    lods:Vec<LOD>,
    description:String,
}

impl Mesh{
    pub fn new(input_mesh:Rc<input::Mesh>) -> Self{
        let name=input_mesh.name.clone();
        let full_vertex_format=input_mesh.full_vertex_format.clone();

        let mut lods=Vec::with_capacity(2);
        for input_lod in input_mesh.lods.iter(){
            let lod=LOD::new(input_lod.clone());

            lods.push(lod);
        }

        Mesh{
            input_mesh:input_mesh,

            name:name,
            full_vertex_format:full_vertex_format,
            include:true,
            display:true,

            lods:lods,
            description:String::new(),
        }
    }

    pub fn render(&self){
        if !self.display || !self.include {
            return;
        }

        for lods in self.lods.iter(){
            //render and break
            //вот думаю надо именно опред индекс брать из УПООРЯДОЧ. Массива
        }
    }
}
