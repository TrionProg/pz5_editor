use std;
use pz5;
use pz5_collada;

use pz5_collada::from_collada::FromColladaLOD;

use Error;
use super::Geometry;

pub struct LOD{
    pub id:usize,
    pub distance:f32,
    pub vertices_count:usize,
    pub geometry:Geometry,
}

impl FromColladaLOD for LOD{
    type Error=Error;
}

//TODO:move Source and VeriexIndexes from collada to pz5? А не, пусть буде
