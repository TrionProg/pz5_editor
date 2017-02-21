use std;
use pz5;
use pz5_collada;
use collada;

use LOD;

enum Parent<'a>{
    ColladaVirtualMesh(pz5_collada::VirtualMesh<'a>),
}

pub struct Mesh{
    pub name:String,
    pub semantics:String,
    pub geometry_type:GeometryType,
    pub lods:Vec<LOD>,
}
/*
impl ToPz5Mesh for Mesh{
    type LOD=LOD;

    fn get_name(&self) -> &String{
        &self.name
    }

    fn get_semantics(&self) -> &String{
        &self.semantics
    }

    fn get_lods(&self) -> &Vec<Self::LOD>{
        &self.lods
    }

    fn get_geometry_type(&self) -> GeometryType{
        self.geometry_type
    }

    /*
    fn write<WriteTo:std::io::Write>(&self,write_to:&mut WriteTo) -> Result<(),pz5::Error>{
        Ok(())
    }


    fn read<ReadFrom:std::io::Read>(read_from:&ReadFrom) -> Result<Self,pz5::Error>{
        Ok(Mesh{name:String::from("hello"),semantics:String::from("semantics"),geometry_type:GeometryType::Points,lods:Vec::new()})
    }
    */

    fn print(&self){
        println!("Mesh {}",self.name);
        println!("Semantics {}",self.semantics);
        println!("Geometry type {}", self.geometry_type.print());

        for lod in self.lods.iter(){
            lod.print();
        }
    }
}

impl FromColladaMesh for Mesh{
    type LOD=LOD;
    type Error=pz5_collada::Error;
}
*/
