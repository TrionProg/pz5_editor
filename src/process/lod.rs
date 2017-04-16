use std;
use pz5;
use glium;
use from_collada;
use render;

use std::sync::Arc;
use std::sync::{Mutex,RwLock};

use pz5::vertex_format::VertexFormat;

use object_pool::multithreaded_growable::{ID,Slot};

use location::Matrix4;

use super::Error;
use super::Storage;
use super::Geometry;

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

        object:&Storage
    ) -> Result< Arc<Self> ,Error > {
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

    pub fn build(
        virtual_lod:&from_collada::VirtualLOD,
        object:&Storage,
        vertex_format:String
    ) -> Result<Arc<Self>,Error> {
        let geometry=from_collada::Geometry::new( virtual_lod.geometry.clone() );

        let lod=LOD::new(
            virtual_lod.distance,
            vertex_format,
            Geometry::ColladaGeometry(geometry),
            virtual_lod.vertices_count,
            String::new(),

            object,
        )?;

        Ok(lod)
    }

    pub fn render(&self, frame:&mut render::Frame) -> Result<(),render::Error> {
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
            None => return Err(render::Error::NoGeometryWithID(geometry_id)),
        };

        geometry.render(frame)?;

        Ok(())
    }

    /*

    pub fn build_render_lod(&mut self, render:&Render, vf:&VertexFormat, vf_str:&String, geometry_type:pz5::GeometryType) -> Result<(),Error> {
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
