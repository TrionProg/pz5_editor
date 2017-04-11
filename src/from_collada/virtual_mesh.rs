use std;
use pz5;
use collada;

use super::Error;
use super::VirtualLOD;
use location::Location;

pub struct VirtualMesh<'a>{
    pub name:String,
    pub vertex_format:String,
    pub location:Location,

    pub lods:Vec<VirtualLOD<'a>>,
    pub geometry_type:pz5::GeometryType,
    pub controller:collada::Controller,
}

impl<'a> VirtualMesh<'a>{
    pub fn check(&self) -> Result<(),Error>{
        if self.lods[0].distance!=0.0 {
            return Err( Error::Other( format!("Mesh \"{}\" must have LOD with 0 distance", self.name) ) );
        }

        for (lod1,lod2) in self.lods.iter().zip(self.lods.iter().skip(1)){
            if lod1.distance==lod2.distance {
                return Err( Error::Other( format!("Mesh \"{}\" has LODS with same distance", self.name) ) );
            }
        }
        Ok(())
    }
}
