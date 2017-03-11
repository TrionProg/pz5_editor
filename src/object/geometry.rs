use std;
use pz5;
use pz5_collada;
use render;
use render::vertex;

use std::rc::Rc;

use ProcessError;
//use Render;
//use render::{LOD,LODTrait};
use pz5::GeometryType;
use pz5::vertex_format::VertexFormat;
use pz5::Pz5Geometry;

pub enum Geometry{
    ColladaGeometry(pz5_collada::from_collada::Geometry),
    //Pz5Geometry(pz5::Pz5Geometry),
}


impl Geometry{
    pub fn build_render_lod(&self, in_vertex_format:&String, out_vertex_format:&String, in_geometry_type:GeometryType, out_geometry_type:GeometryType) -> Result<Pz5Geometry, ProcessError>{
        //TODO:adapt out_vertex_format

        match *self{
            Geometry::ColladaGeometry( ref geometry ) => {
                let pz5_geometry=geometry.build_geometry( &VertexFormat::parse(out_vertex_format)? )?;
                Ok(pz5_geometry)

                /*
                match out_vf_str.as_str() {
                    //"VERTEX:(X:f32,Y:f32)" => LOD::<vertex::VertexP2>::new(render, out_vf_str, pz5_geometry, geometry_type)?,
                    "VERTEX:(X:f32,Y:f32,Z:f32) NORMAL:(X:f32,Y:f32,Z:f32)" => LOD::<vertex::VertexP3N3>::new(render, out_vf_str, pz5_geometry, geometry_type)?,
                    //"VERTEX:(X:float,Y:float)" => LOD::new(render, out_vf_str, pz5_geometry, geometry_type),
                    _ => return Err( ProcessError::NoShaderProgram(out_vf_str.clone()) ),
                }
                */
            },
        }
    }
}

//fn to_render_lod(&self, in_vf, out_vf, render) -> Result<Box<LODTrait>, Err>
