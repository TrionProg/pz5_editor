use std;
use pz5;
use collada;
use pz5_collada;

use std::rc::Rc;

pub enum Geometry{
    ColladaGeometry(Rc<collada::Mesh>),//IDEA:maybe store fvf here?
    Pz5Geometry(pz5::Pz5Geometry),
}

//fn to_render_lod(&self, in_fvf, out_fvf, render) -> Result<Box<LODTrait>, Err>
