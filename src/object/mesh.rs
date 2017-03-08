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
use ID;
use Object;
use RenderSender;
use RenderTask;
use SlabElement;

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
    pub id:ID,
    pub vertex_format:String,
    pub geometry_type:GeometryType,

    pub model:Mutex< Option<ID> >,
    pub lods:RwLock< Vec<Arc<LOD>> >,

    pub attrib:RwLock< MeshAttrib >,
}

impl FromColladaMesh for Mesh{
    type LOD=LOD;
    type Error=Error;
    type Container=Arc<Self>;
}

impl SlabElement for Mesh{
    fn set_id(&mut self,id:ID) {
        self.id=id;
    }

    fn get_id(&self) -> ID {
        self.id
    }
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
            id:ID::zeroed(),
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

    pub fn add_lod(&self, lod:Arc<LOD>, to_render_tx:&RenderSender) -> Result<(),Error>{
        *lod.mesh.lock().unwrap()=Some(self.id);

        let mut lods_guard=self.lods.write().unwrap();

        lods_guard.push(lod.clone());
        lods_guard.sort_by(|lod1,lod2| {
            let dist1=lod1.attrib.read().unwrap().distance;
            let dist2=lod2.attrib.read().unwrap().distance;

            dist1.partial_cmp(&dist2).unwrap()
        });

        let to_vertex_format=self.attrib.read().unwrap().vertex_format.clone();
        let to_geometry_type=self.attrib.read().unwrap().geometry_type;

        let pz5_geometry=lod.geometry.build_render_lod(&lod.vertex_format, &to_vertex_format, self.geometry_type, to_geometry_type)?;

        to_render_tx.send( RenderTask::LoadLOD(lod,pz5_geometry) );

        Ok(())
    }

    pub fn remove_lod(&self, lod:&Arc<LOD>, to_render_tx:&RenderSender){
        *lod.mesh.lock().unwrap()=None;

        if lod.render_lod.lock().unwrap().is_some() {
            to_render_tx.send( RenderTask::RemoveLOD(lod.clone()) );
        }

        let mut lods_guard=self.lods.write().unwrap();

        let index={
            let mut index=0;

            for (i,l) in lods_guard.iter().enumerate(){
                if l.id==lod.id {//this lod
                    index=i;
                    break;
                }
            }

            index
        };

        lods_guard.remove(index);
    }

    //rebuild on vf change

    /*

    pub fn build_render_lods(&self, to_render_tx:&RenderSender) -> Result<(),Error> {
        let vertex_format=self.adapt_vertex_format()?;

        let lods_guard=self.lods.read().unwrap();

        for lod in lods_guard.iter(){
            lod.build_render_lod(&self,to_render_tx);
        }
    }
    */

    pub fn adapt_vertex_format(&self) -> Result<String,Error> {
        Ok( self.vertex_format.clone() )
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
