use std;
use pz5;
use collada;
use pz5_collada;

use std::rc::Rc;

pub enum Geometry{
    ColladaGeometry(Rc<collada::Mesh>),
    Pz5Geometry(pz5::Pz5Geometry),
}
