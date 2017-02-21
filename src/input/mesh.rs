use std;
use pz5;
use pz5_collada;

use std::rc::Rc;

use pz5_collada::from_collada::FromColladaMesh;
use std::collections::HashMap;

use Error;

use super::LOD;

pub struct Mesh{
    pub name:String,
    pub vertex_full_format:String,
    pub geometry_type:pz5::GeometryType,
    pub lods:Vec<Rc<LOD>>,
}

impl FromColladaMesh for Mesh{
    type LOD=LOD;
    type Container=Rc<LOD>;
    type Error=Error;

    fn get_name(&self) -> &String{
        &self.name
    }
}
