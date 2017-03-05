use std;
use pz5;
use pz5_collada;
use glium;
use render;

use std::sync::Arc;
use std::sync::{Mutex,RwLock};

use pz5_collada::from_collada::FromColladaMesh;
use std::collections::HashMap;
use pz5::vertex_format::VertexFormat;
use pz5::GeometryType;

use Error;
use Object;

use super::LOD;
//use ObjectFrame;

pub struct MeshAttrib{
    pub name:String,
    pub vertex_format:String,
    pub geometry_type:GeometryType,
    pub description:String,

    pub include:bool,
    pub display:bool,
}

pub struct Mesh{
    pub id:usize,
    pub vertex_format:String,
    pub geometry_type:GeometryType,

    pub model:Mutex< Option<usize> >,
    pub lods:RwLock< Vec<Arc<LOD>> >,

    pub attrib:RwLock< MeshAttrib >,
}

impl FromColladaMesh for Mesh{
    type LOD=LOD;
    type Error=Error;
    type Container=Arc<Self>;
}

impl Mesh{
    pub fn new(
        name:String,
        vertex_format:String,
        geometry_type:GeometryType,
        description:String,

        object:&Object
    ) -> Result< Arc<Self>, Error >{
        let mesh=Mesh{
            id:0,
            vertex_format:vertex_format.clone(),
            geometry_type:geometry_type.clone(),

            model:Mutex::new( None ),
            lods:RwLock::new( Vec::new() ),

            attrib:RwLock::new(
                MeshAttrib{
                    name:name,
                    vertex_format:vertex_format,
                    geometry_type:geometry_type,
                    description:description,

                    include:true,
                    display:object.is_gui,
                }
            ),
        };

        object.add_mesh_to_list( mesh )
    }

    pub fn add_lod(&self, lod:Arc<LOD>){
        {
            let mut lod_mesh=lod.mesh.lock().unwrap();

            if *lod_mesh!=None {
                //выписать lod
            }
            *lod_mesh=Some(self.id); //Can take Weak by arc in object.meshes
        }

        let mut lods_guard=self.lods.write().unwrap();

        lods_guard.push(lod);
        lods_guard.sort_by(|lod1,lod2| {
            let dist1=lod1.attrib.read().unwrap().distance;
            let dist2=lod2.attrib.read().unwrap().distance;

            dist1.partial_cmp(&dist2).unwrap()
        });
    }
/*

    pub fn build_render_lods(&mut self, render:&Render) -> Result<(),Error> {
        let vertex_format_str=String::from("VERTEX:(X:f32,Y:f32,Z:f32) NORMAL:(X:f32,Y:f32,Z:f32)");
        let vertex_format=VertexFormat::parse(&vertex_format_str).unwrap();

        for lod in self.lods.iter_mut(){
            Rc::get_mut(lod).unwrap().build_render_lod(render, &vertex_format, &vertex_format_str, self.geometry_type)?;
        }

        Ok(())
    }

    pub fn render(&self, frame:&mut ObjectFrame) -> Result<(),glium::DrawError>{
        if !self.display {
            return Ok(());
        }

        self.lods[0].render(frame)?;

        Ok(())
    }

    pub fn adapt_vertex_format(in_fvf:&String) -> Result<String,Error> {
        Ok( in_fvf.clone() )
    }
*/
}
