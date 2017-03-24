use std;
use pz5;
use glium;
use collada;

use std::sync::Arc;
use std::sync::{Mutex,RwLock};

use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::path::Path;

use from_collada::VirtualModel;
use from_collada::VirtualMesh;

use object_pool::multithreaded_growable::{ID,Slot};

use ProcessError;
use Object;
use RenderSender;

use super::LOD;
use super::Mesh;
use super::Geometry;

use RenderError;
use RenderFrame;

pub struct ModelAttrib{
    pub name:String,
    pub description: String,

    pub include: bool,
    pub display: bool,
}

pub struct Model{
    pub id:ID,

    pub meshes: RwLock< HashMap<String, Arc<Mesh>> >,

    pub attrib:RwLock< ModelAttrib >,
}

impl Slot for Model{
    fn set_id(&mut self,id:ID) {
        self.id=id;
    }

    fn get_id(&self) -> ID {
        self.id
    }
}

impl Model{
    pub fn new(
        name:String,
        description:String,

        object:&Object
    ) -> Result< Arc<Self>, ProcessError >{
        let model=Model{
            id:ID::zeroed(),

            meshes:RwLock::new( HashMap::new() ),

            attrib:RwLock::new(
                ModelAttrib{
                    name:name,
                    description:description,

                    include:true,
                    display:object.is_gui,
                }
            ),
        };

        object.add_model_to_pool(model)
    }

    pub fn add_mesh(&self, mesh:Arc<Mesh>){
        {
            let mut mesh_model=mesh.model.lock().unwrap();

            if *mesh_model!=None {
                //выписать mesh
            }
            *mesh_model=Some(self.id); //Can take Weak by arc in object.models
        }

        let mut meshes_guard=self.meshes.write().unwrap();

        let mut cnt=0;
        let base_name=mesh.attrib.read().unwrap().name.clone();
        let mut name=base_name.clone();

        loop{
            match meshes_guard.entry(name.clone()) {
                Entry::Vacant(e) => {
                    e.insert(mesh);
                    break;
                },
                Entry::Occupied(_) => {
                    cnt+=1;
                    name=format!("{}.{}",base_name,cnt);
                    mesh.attrib.write().unwrap().name=name.clone();
                }
            }
        }
    }

    //remove_mesh

    pub fn get_model_name(file_name:&Path) -> Result<String,ProcessError>{
        let model_name=match file_name.file_name() {
            Some( file_name_os_str ) => {
                match file_name_os_str.to_str() {
                    Some( file_name_str ) => String::from(file_name_str),//TODO:remove extension
                    None => return Err( ProcessError::FileNameNotUTF ),
                }
            },
            None => return Err( ProcessError::NoFileName ),
        };

        Ok(model_name)
    }

    pub fn load_from_collada(file_name:&Path, object:&Object, to_render_tx:&RenderSender) -> Result<(),ProcessError>{
        let model_name=Self::get_model_name(file_name)?;
        let document=VirtualModel::parse_collada(file_name)?;
        let virtual_meshes=VirtualModel::generate_virtual_meshes(&document)?;

        let model=Model::build(&document, &virtual_meshes, model_name, object, to_render_tx)?;

        object.add_model(model);

        Ok(())
    }

    pub fn build(
        document:&collada::Document,
        virtual_meshes:&HashMap<String,VirtualMesh>,
        model_name:String,
        object:&Object,
        to_render_tx:&RenderSender
    ) -> Result<Arc<Self>,ProcessError> {
        let model=Model::new(
            model_name,
            String::new(),

            object
        )?;

        for (_,virtual_mesh) in virtual_meshes.iter(){
            println!("{} {} {}",virtual_mesh.position, virtual_mesh.rotation, virtual_mesh.scale);
            let mesh=Mesh::build(virtual_mesh,object,to_render_tx)?;

            model.add_mesh(mesh);
        }

        Ok(model)
    }

    pub fn render(&self, frame:&mut RenderFrame) -> Result<(),RenderError> {
        {
            let attrib=self.attrib.read().unwrap();

            if !attrib.include || !attrib.display {
                return Ok(());
            }
        }

        let meshes_guard=self.meshes.read().unwrap();

        for (_,mesh) in meshes_guard.iter() {
            mesh.render(frame)?;
        }

        Ok(())
    }

/*
                }

    pub fn include_collada_model(file_name:&Path, object:&mut Object>) -> Result<Self,ProcessError> {
        let model_name=Self::get_model_name(file_name)?;

        Model::new(
            model_name,
            String::new(),
            object.is_gui,
        )

        let model=<Self as FromColladaModel>::build(file_name,
        |document, virtual_meshes|{
            let mut meshes=HashMap::new();

            for (_,virtual_mesh) in virtual_meshes.iter(){
                let mesh=Mesh::build(virtual_mesh,|virtual_mesh|{
                    //let vertex_format=Mesh::adapt_vertex_format(&virtual_mesh.vertex_format)?;
                    //remove & from vf
                    //adapt and use adapted vf for lods

                    let vertex_format=String::from("VERTEX:(X:f32,Y:f32)");

                    Mesh::new(
                        virtual_mesh.name.clone(),
                        virtual_mesh.vertex_format.clone(),
                        vertex_format,
                        virtual_mesh.geometry_type,
                        lods,
                        String::new(),
                        render.clone(),
                    )

                    let mut lods=Vec::with_capacity(virtual_mesh.lods.len());

                    for virtual_lod in virtual_mesh.lods.iter(){
                        let lod = LOD::build(virtual_lod,|virtual_lod,geometry|{
                            let id=geometry.collada_mesh.id;

                            LOD::new(
                                virtual_lod.distance,
                                id,
                                Geometry::ColladaGeometry(geometry),
                                virtual_lod.vertices_count,
                                String::new(),
                                render.is_some(),
                            )
                        })?;

                        lods.push(Arc::new(lod));
                    }

                    Mesh::new(
                        virtual_mesh.name.clone(),
                        virtual_mesh.vertex_format.clone(),
                        vertex_format,
                        virtual_mesh.geometry_type,
                        lods,
                        String::new(),
                        render.clone(),
                    )
                })?;

                match meshes.entry(mesh.name.clone()){
                    Entry::Occupied( _ ) => return Err(Self::ProcessError::from(
                        ProcessError::Other( format!("Mesh \"{}\" already exists",mesh.name) )
                    )),
                    Entry::Vacant( e ) => {e.insert(Arc::new(mesh));},
                }
            }

            Model::new(
                model_id,
                model_name,
                meshes,
                String::new(),
                render.is_some(),
            )
        })?;

        Ok(model)
    }

    pub fn render(&self, frame:&mut ObjectFrame) -> Result<(),glium::DrawProcessError>{
        if !self.display {
            return Ok(());
        }

        for (_,mesh) in self.meshes.iter(){
            mesh.render(frame)?;
        }

        Ok(())
    }
*/
}
