use std;
use pz5;
use glium;
use cgmath;
use render;

use std::sync::Arc;
use std::sync::{Mutex,RwLock};

use std::collections::HashMap;
use pz5::vertex_format::VertexFormat;
use pz5::GeometryType;
use from_collada::VirtualMesh;

use cgmath::Matrix4;

use object_pool::multithreaded_growable::{ID,Slot};

use super::Error;
use super::Storage;

use location::Location;
use location::calculate_matrix;

use super::LOD;

pub struct MeshAttrib{
    pub name:String,
    pub vertex_format:String,
    pub geometry_type:GeometryType,
    pub description:String,

    pub location:Location,

    pub include:bool,
    pub display:bool,
}

pub struct Mesh{
    pub id:ID,
    pub vertex_format:String,
    pub geometry_type:GeometryType,

    pub matrix:Mutex< Matrix4<f32> >,
    pub model:Mutex< Option<ID> >,
    pub lods:RwLock< Vec<Arc<LOD>> >,

    pub attrib:RwLock< MeshAttrib >,
}

impl Slot for Mesh{
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

        location:Location,

        object:&Storage
    ) -> Result< Arc<Self>, Error >{
        use cgmath::SquareMatrix;

        let mesh=Mesh{
            id:ID::zeroed(),
            vertex_format:vertex_format.clone(),
            geometry_type:geometry_type.clone(),

            matrix:Mutex::new( Matrix4::identity() ),
            model:Mutex::new( None ),
            lods:RwLock::new( Vec::new() ),
            //matrix:Mutex

            attrib:RwLock::new(
                MeshAttrib{
                    name:name,
                    vertex_format:vertex_format,
                    geometry_type:geometry_type,
                    description:description,

                    location:location,

                    include:true,
                    display:object.is_gui,
                }
            ),
        };

        mesh.calc_matrix();

        object.add_mesh_to_pool( mesh )
    }

    pub fn build(
        virtual_mesh:&VirtualMesh,
        object:&Storage,
        to_render_tx:&render::Sender
    ) -> Result<Arc<Self>,Error> {
        let mesh=Mesh::new(
            virtual_mesh.name.clone(),
            virtual_mesh.vertex_format.clone(),
            virtual_mesh.geometry_type,
            String::new(),

            virtual_mesh.location,

            object
        )?;

        for virtual_lod in virtual_mesh.lods.iter(){
            let lod=LOD::build(virtual_lod, object, virtual_mesh.vertex_format.clone() )?;

            mesh.add_lod(lod, to_render_tx)?;
        }

        Ok(mesh)
    }

    pub fn add_lod(&self, lod:Arc<LOD>, to_render_tx:&render::Sender) -> Result<(),Error>{
        *lod.mesh.lock().unwrap()=Some(self.id);

        let mut lods_guard=self.lods.write().unwrap();

        lods_guard.push(lod.clone());
        lods_guard.sort_by(|lod1,lod2| {
            let dist1=lod1.attrib.read().unwrap().distance;
            let dist2=lod2.attrib.read().unwrap().distance;

            dist1.partial_cmp(&dist2).unwrap()
        });
        //TODO:adapt format
        let to_vertex_format=self.adapt_vertex_format()?;//self.attrib.read().unwrap().vertex_format.clone();
        let to_geometry_type=self.attrib.read().unwrap().geometry_type;

        let pz5_geometry={
            let matrix_guard=self.matrix.lock().unwrap();
            //lod.geometry.build_render_lod(&*matrix_guard, &lod.vertex_format, &to_vertex_format, self.geometry_type, to_geometry_type)?
            lod.geometry.build_render_lod(&*matrix_guard, &to_vertex_format)?
        };


        to_render_tx.send( render::Task::LoadLOD(lod,pz5_geometry,to_vertex_format,to_geometry_type) )?;

        Ok(())
    }

    pub fn remove_lod(&self, lod:&Arc<LOD>, to_render_tx:&render::Sender) -> Result<(),Error> {
        *lod.mesh.lock().unwrap()=None;

        match *lod.geometry_id.lock().unwrap() {
            Some( ref geometry_id ) =>
                to_render_tx.send( render::Task::RemoveLOD(lod.clone(),*geometry_id) )?,
            None => {},
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

        Ok(())
    }

    pub fn render(&self, frame:&mut render::Frame) -> Result<(),render::Error> {
        {
            let attrib=self.attrib.read().unwrap();

            if !attrib.include || !attrib.display {
                return Ok(());
            }
        }

        let lods_guard=self.lods.read().unwrap();
        (*lods_guard)[0].render(frame)?;

        /*
        for lod in lods_guard.iter() {
            lod.render(&self, frame)?;
        }
        */

        Ok(())
    }

    pub fn calc_matrix(&self) {
        let attrib_guard=self.attrib.read().unwrap();
        *self.matrix.lock().unwrap()=calculate_matrix(&attrib_guard.location);
    }

    //rebuild on vf change

    /*

    pub fn build_render_lods(&self, to_render_tx:&render::Sender) -> Result<(),Error> {
        let vertex_format=self.adapt_vertex_format()?;

        let lods_guard=self.lods.read().unwrap();

        for lod in lods_guard.iter(){
            lod.build_render_lod(&self,to_render_tx);
        }
    }
    */

    pub fn adapt_vertex_format(&self) -> Result<String,Error> {
        //Ok( self.vertex_format.clone() )
        Ok(String::from("VERTEX:(X:f32,Y:f32,Z:f32) NORMAL:(X:f32,Y:f32,Z:f32)"))
    }
}
