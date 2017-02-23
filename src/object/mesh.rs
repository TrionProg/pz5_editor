use std;
use pz5;
use pz5_collada;

use std::rc::Rc;

use pz5_collada::from_collada::FromColladaMesh;
use std::collections::HashMap;

use Error;

use super::LOD;

pub struct Mesh{
    key_name:String,
    in_full_vertex_format:String,

    pub name:String,
    pub full_vertex_format:String,
    pub geometry_type:pz5::GeometryType,
    pub lods:Vec<Rc<LOD>>,
    pub description:String,

    pub include:bool,
    pub display:bool,
}

impl FromColladaMesh for Mesh{
    type LOD=LOD;
    type Container=Rc<LOD>;
    type Error=Error;

    fn get_name(&self) -> &String{
        &self.name
    }
}

impl Mesh{
    pub fn new(
        name:String,
        in_full_vertex_format:String,
        full_vertex_format:String,
        geometry_type:pz5::GeometryType,
        lods:Vec<Rc<LOD>>,
        description:String,
        display:bool,
    ) -> Result<Self, Error>{
        let key_name=name.clone();

        let mesh=Mesh{
            key_name:key_name,
            in_full_vertex_format:in_full_vertex_format,

            name:name,
            full_vertex_format:full_vertex_format,
            geometry_type:geometry_type,
            lods:lods,
            description:description,

            include:true,
            display:display,
        };

        Ok( mesh )
    }

    pub fn adapt_vertex_format(in_fvf:&String) -> Result<String,Error> {
        Ok( in_fvf.clone() )
    }
}
