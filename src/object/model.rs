use std;
use pz5;
use pz5_collada;
use glium;

use std::sync::Arc;
use std::sync::{Mutex,RwLock};

use pz5_collada::from_collada::FromColladaModel;
use pz5_collada::from_collada::FromColladaMesh;
use pz5_collada::from_collada::FromColladaLOD;

use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::path::Path;

use Error;
use Object;
//use ObjectFrame;

use super::LOD;
use super::Mesh;
use super::Geometry;

pub struct ModelAttrib{
    pub name:String,
    pub description: String,

    pub include: bool,
    pub display: bool,
}

pub struct Model{
    pub id:usize,

    pub meshes: RwLock< HashMap<String, Arc<Mesh>> >,

    pub attrib:RwLock< ModelAttrib >,
}

impl FromColladaModel for Model{
    type Mesh=Mesh;
    type Error=Error;
    type Container=Arc<Self>;
}

impl Model{
    pub fn new(
        name:String,
        description:String,

        object:&Object
    ) -> Result< Arc<Self>, Error >{
        let model=Model{
            id:0,

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

        object.add_model_to_list(model)
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
                    let name=format!("{}.{}",base_name,cnt);
                    mesh.attrib.write().unwrap().name=name.clone();
                }
            }
        }
    }

    pub fn get_model_name(file_name:&Path) -> Result<String,Error>{
        let model_name=match file_name.file_name() {
            Some( file_name_os_str ) => {
                match file_name_os_str.to_str() {
                    Some( file_name_str ) => String::from(file_name_str),//TODO:remove extension
                    None => return Err( Error::FileNameNotUTF ),
                }
            },
            None => return Err( Error::NoFileName ),
        };

        Ok(model_name)
    }

    pub fn load_from_collada(file_name:&Path, object:&Object) -> Result<(),Error>{
        let model_name=Self::get_model_name(file_name)?;

        let model=Model::build(file_name,|document, virtual_meshes|{
            let model=Model::new(
                model_name,
                String::new(),

                object
            )?;

            for (_,virtual_mesh) in virtual_meshes.iter(){
                let mesh=Mesh::build(virtual_mesh,|virtual_mesh|{
                    let mesh=Mesh::new(
                        virtual_mesh.name.clone(),
                        virtual_mesh.vertex_format.clone(),
                        virtual_mesh.geometry_type,
                        String::new(),

                        object
                    )?;

                    for virtual_lod in virtual_mesh.lods.iter(){
                        let lod=LOD::build(virtual_lod,|virtual_lod,geometry|{
                            let lod=LOD::new(
                                virtual_lod.distance,
                                Geometry::ColladaGeometry(geometry),
                                virtual_lod.vertices_count,
                                String::new(),

                                object,
                            )?;

                            Ok(lod)
                        })?;

                        mesh.add_lod(lod);
                    }

                    Ok(mesh)
                })?;

                model.add_mesh(mesh);
            }

            Ok(model)
        })?;

        object.add_model(model);

        Ok(())
    }

/*
                }

    pub fn include_collada_model(file_name:&Path, object:&mut Object>) -> Result<Self,Error> {
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
                    Entry::Occupied( _ ) => return Err(Self::Error::from(
                        Error::Other( format!("Mesh \"{}\" already exists",mesh.name) )
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

    pub fn render(&self, frame:&mut ObjectFrame) -> Result<(),glium::DrawError>{
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
