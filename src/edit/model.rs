use std;
use pz5;
use pz5_collada;
use input;

use std::rc::Rc;

use super::Mesh;

pub struct Model{
    input_model:Rc<input::Model>,
    name:String,
    meshes:Vec<Mesh>,
    description:String,
}

impl Model{
    pub fn new(input_model:Rc<input::Model>) -> Self{
        let name=input_model.name.clone();

        let mut meshes=Vec::new();

        for (_,input_mesh) in input_model.meshes.iter(){
            let mesh=Mesh::new(input_mesh.clone());

            meshes.push(mesh);
        }

        Model{
            input_model:input_model,
            name:name,
            meshes:meshes,
            description:String::new(),
        }
    }
}
