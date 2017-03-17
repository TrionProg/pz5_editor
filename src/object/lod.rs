use std;
use pz5;
use pz5_collada;
use glium;
use render;

use std::sync::Arc;
use std::sync::{Mutex,RwLock};

use pz5_collada::from_collada::FromColladaLOD;
use pz5::vertex_format::VertexFormat;

use object_pool::multithreaded_growable::{ID,Slot};

use cgmath::Matrix4;

use ProcessError;
use Object;

use super::Geometry;

use RenderError;
use RenderFrame;

pub struct LODAttrib{
    pub distance:f32,
    pub vertices_count:usize,
    pub description:String,

    pub include:bool,
    pub display:bool,
}

pub struct LOD{
    pub id:ID,
    pub vertices_count:usize,
    pub geometry:Geometry,
    pub vertex_format:String,

    pub mesh:Mutex< Option<ID> >,
    pub geometry_id:Mutex< Option<ID> >,

    pub attrib:RwLock< LODAttrib >,
}

impl FromColladaLOD for LOD{
    type Error=ProcessError;
    type Container=Arc<Self>;
}

impl Slot for LOD{
    fn set_id(&mut self,id:ID) {
        self.id=id;
    }

    fn get_id(&self) -> ID {
        self.id
    }
}

impl LOD{
    pub fn new(
        distance:f32,
        vertex_format:String,
        geometry:Geometry,
        vertices_count:usize,
        description:String,

        object:&Object
    ) -> Result< Arc<Self> ,ProcessError > {
        let lod=LOD{
            id:ID::zeroed(),
            vertices_count:vertices_count,
            geometry:geometry,
            vertex_format:vertex_format,

            mesh:Mutex::new( None ),
            geometry_id:Mutex::new( None ),

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

        object.add_lod_to_pool( lod )
    }

    pub fn render(&self, frame:&mut RenderFrame, mesh_matrix:&Matrix4<f32>) -> Result<(),RenderError> {
        {
            let attrib=self.attrib.read().unwrap();

            if !attrib.include || !attrib.display {
                return Ok(());
            }
        }

        let mut geometry_id_guard=self.geometry_id.lock().unwrap();

        let geometry_id = match *geometry_id_guard {
            Some( ref geometry_id ) => *geometry_id,
            None => return Ok(()),
        };

        let geometry=match frame.storage.geometries.get(geometry_id) {
            Some( geometry ) => geometry,
            None => return Err(RenderError::NoGeometryWithID(geometry_id)),
        };

        geometry.render(frame, mesh_matrix)?;

        Ok(())
    }

    /*

    pub fn build_render_lod(&mut self, render:&Render, vf:&VertexFormat, vf_str:&String, geometry_type:pz5::GeometryType) -> Result<(),ProcessError> {
        let render_lod=self.geometry.build_render_lod(render, vf, vf_str, geometry_type)?;

        self.render_lod=Some(render_lod);

        Ok(())
    }

    pub fn render(&self, frame:&mut ObjectFrame) -> Result<(),glium::DrawProcessError>{
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
