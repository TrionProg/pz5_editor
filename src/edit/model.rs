use std;
use pz5;
use pz5_collada;
use input;

use std::rc::Rc;

use super::Mesh;

pub struct Model{
    id:usize,
    input_model:Rc<input::Model>,

    name:String,
    include:bool,
    display:bool,

    meshes:Vec<Mesh>,
    description:String,
}

impl Model{
    pub fn new(input_model:Rc<input::Model>, id:usize) -> Self{
        let name=input_model.name.clone();

        let mut meshes=Vec::new();

        for (_,input_mesh) in input_model.meshes.iter(){
            let mesh=Mesh::new(input_mesh.clone());

            meshes.push(mesh);
        }

        Model{
            id:id,
            input_model:input_model,

            name:name,
            include:true,
            display:true,

            meshes:meshes,
            description:String::new(),
        }
    }

    pub fn render(&self){
        if !self.display || !self.include {
            return;
        }

        for mesh in self.meshes.iter(){
            mesh.render();
        }
    }
}
