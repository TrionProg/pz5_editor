use std;
use pz5;
use pz5_collada;
use glium;
use render;

use std::sync::Arc;
use std::sync::{Mutex,RwLock};

use pz5_collada::from_collada::FromColladaLOD;
use pz5::vertex_format::VertexFormat;

use Error;
use Object;

use super::Geometry;
//use ObjectFrame;

pub struct LODAttrib{
    pub distance:f32,
    pub vertices_count:usize,
    pub description:String,

    pub include:bool,
    pub display:bool,
}

pub struct LOD{
    pub id:usize,
    pub vertices_count:usize,
    pub geometry:Geometry,

    pub mesh:Mutex< Option<usize> >,
    pub render_lod:Mutex< Option<usize> >,

    pub attrib:RwLock< LODAttrib >,
}

impl FromColladaLOD for LOD{
    type Error=Error;
    type Container=Arc<Self>;
}

impl LOD{
    pub fn new(
        distance:f32,
        geometry:Geometry,
        vertices_count:usize,
        description:String,

        object:&Object
    ) -> Result< Arc<Self> ,Error > {
        let lod=LOD{
            id:0,
            vertices_count:vertices_count,
            geometry:geometry,

            mesh:Mutex::new( None ),
            render_lod:Mutex::new( None ),

            attrib:RwLock::new(
                LODAttrib{
                    distance:distance,
                    vertices_count:vertices_count,
                    description:description,

                    include:true,
                    display:object.is_gui,
                }
            ),
        };

        object.add_lod_to_list( lod )
    }

    /*

    pub fn build_render_lod(&mut self, render:&Render, vf:&VertexFormat, vf_str:&String, geometry_type:pz5::GeometryType) -> Result<(),Error> {
        let render_lod=self.geometry.build_render_lod(render, vf, vf_str, geometry_type)?;

        self.render_lod=Some(render_lod);

        Ok(())
    }

    pub fn render(&self, frame:&mut ObjectFrame) -> Result<(),glium::DrawError>{
        if !self.display {
            return Ok(());
        }

        match self.render_lod{
            Some( ref render_lod ) => render_lod.render( frame ),
            None => Ok(())
        }
    }
    */

}
