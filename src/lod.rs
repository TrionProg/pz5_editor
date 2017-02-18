use std;
use pz5;
use pz5_collada;

use pz5::ToPz5LOD;

use pz5_collada::FromColladaLOD;

pub struct LOD{
    pub distance:f32,
    pub geometry:Vec<u8>,
    pub vertices_count:usize,
}

impl ToPz5LOD for LOD{
    fn get_distance(&self) -> f32{
        self.distance
    }

    fn get_data(&self) -> &[u8]{
        &self.geometry[8..]
    }

    fn get_all_data(&self) -> &[u8]{
        &self.geometry[..]
    }

    fn get_vertices_count(&self) -> usize{
        self.vertices_count
    }

    /*
    fn write<WriteTo:std::io::Write>(&self,write_to:&mut WriteTo) -> Result<(),pz5::Error>{
        Ok(())
    }

    fn read<ReadFrom:std::io::Read>(read_from:&ReadFrom) -> Result<Self,pz5::Error>{
        Ok(LOD{distance:0.0,geometry:Vec::new(),vertices_count:0})
    }
    */

    fn print(&self){
        println!("LOD distance {}",self.distance);
        println!("DATA len {}",self.geometry.len());
        println!("Vertex count: {}",self.vertices_count);
    }
}

impl FromColladaLOD for LOD{
    type Error=pz5_collada::Error;
}
