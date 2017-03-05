use std;
use pz5;
use pz5_collada;
use render;
use render::vertex;

use std::rc::Rc;

use Error;
//use Render;
//use render::{LOD,LODTrait};
use pz5::GeometryType;
use pz5::vertex_format::VertexFormat;

pub enum Geometry{
    ColladaGeometry(pz5_collada::from_collada::Geometry),
    //Pz5Geometry(pz5::Pz5Geometry),
}

/*

impl Geometry{
    pub fn build_render_lod(&self, render:&Render, out_vf:&VertexFormat, out_vf_str:&String, geometry_type:GeometryType) -> Result<Box<LODTrait>, Error>{
        let lod=match *self{
            Geometry::ColladaGeometry( ref geometry ) => {
                let pz5_geometry=geometry.build_geometry(out_vf)?;

                match out_vf_str.as_str() {
                    //"VERTEX:(X:f32,Y:f32)" => LOD::<vertex::VertexP2>::new(render, out_vf_str, pz5_geometry, geometry_type)?,
                    "VERTEX:(X:f32,Y:f32,Z:f32) NORMAL:(X:f32,Y:f32,Z:f32)" => LOD::<vertex::VertexP3N3>::new(render, out_vf_str, pz5_geometry, geometry_type)?,
                    //"VERTEX:(X:float,Y:float)" => LOD::new(render, out_vf_str, pz5_geometry, geometry_type),
                    _ => return Err( Error::NoShaderProgram(out_vf_str.clone()) ),
                }
            },
        };

        Ok(lod)
    }
}
*/

//fn to_render_lod(&self, in_vf, out_vf, render) -> Result<Box<LODTrait>, Err>
