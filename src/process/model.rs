use std;
use pz5;
use glium;
use collada;
use render;

use std::sync::Arc;
use std::sync::{Mutex,RwLock};

use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::path::Path;

use from_collada::VirtualModel;
use from_collada::VirtualMesh;

use object_pool::multithreaded_growable::{ID,Slot};

use super::Error;

use super::Storage;
use super::LOD;
use super::Mesh;
use super::Geometry;
use super::Skeleton;
use super::ZeroFrame;
use super::Animation;

pub struct ModelAttrib{
    pub name:String,
    pub description: String,

    pub include: bool,
    pub display: bool,
}

pub struct Model{
    pub id:ID,

    pub meshes: RwLock< HashMap<String, Arc<Mesh>> >,
    pub skeleton: RwLock< Skeleton >,
    pub zero_frame: RwLock< ZeroFrame >,
    //pub animations: RwLock< HashMap<

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

        skeleton:Skeleton,
        zero_frame:ZeroFrame,

        object:&Storage
    ) -> Result< Arc<Self>, Error >{
        let model=Model{
            id:ID::zeroed(),

            meshes:RwLock::new( HashMap::new() ),
            skeleton:RwLock::new( skeleton ),
            zero_frame:RwLock::new( zero_frame ),

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

    pub fn load_from_collada(file_name:&Path, object:&Storage, to_render_tx:&render::Sender) -> Result<(),Error>{
        let model_name=Self::get_model_name(file_name)?;
        let document=VirtualModel::parse_collada(file_name)?;

        for (scene_name, scene) in document.scenes.iter(){
            let mut virtual_models = VirtualModel::generate_virtual_models(&document, scene)?;
            VirtualModel::check_and_sort_virtual_models(&mut virtual_models)?;
            let (virtual_models, virtual_instances) = VirtualModel::separate_to_models_and_instances(virtual_models)?;

            println!("Virtual models");
            for vm in virtual_models.iter() {
                let vm=vm.borrow();
                println!("{:?}",vm.best_name);
                for (oldname,mesh) in vm.meshes.iter() {
                    println!("{} - {}",oldname, mesh.name)
                }
            }

            println!("Virtual instances");
            for vi in virtual_instances.iter() {
                println!("{}",vi.name);
            }

            for virtual_model in virtual_models.iter() {
                let model=Model::from_virtual(&document, &virtual_model.borrow(), object, to_render_tx)?;

                object.add_model(model);
            }
        }

        /*

        document.print();

        println!("SKINS");

        use location::*;
        for (_,skin) in document.skins.iter() {
            println!("{}",skin.geometry_id);

            let location = match Location::from_collada(&skin.bind_location) {
                Ok( loc ) => loc,
                Err( _ ) => panic!("aaa"),//return Err( Error::SkeletonDifferentSizes( skeleton_node.name.clone() )),
            };

            println!("{:?} {:?} /", location.position, location.rotation);

            let bone_matrixes=skin.additional_sources.get(&String::from("INV_BIND_MATRIX")).unwrap().layers.get(&String::from("location")).unwrap();
            let bone_names=skin.additional_sources.get(&String::from("JOINT")).unwrap().layers.get(&String::from("bone_name")).unwrap();

            let bone_names=match *bone_names {
                collada::SourceLayer::Name( ref n ) => n,
                _ => panic!("aa"),
            };

            match *bone_matrixes {
                collada::SourceLayer::Location( ref matrices ) => {
                    for (m,n) in matrices.iter().zip(bone_names.iter()) {
                        let location = match Location::from_collada(m) {
                            Ok( loc ) => loc,
                            Err( _ ) => panic!("aaa"),//return Err( Error::SkeletonDifferentSizes( skeleton_node.name.clone() )),
                        };

                        println!("{}:{:?} {:?}", n,location.position, location.rotation);
                    }
                },
                _ => {},
            }
        }

        let skene=&document.scenes.iter().next().unwrap().1;
        println!("SKELETONS");



        for (_,skeleton_node) in skene.skeletons.iter() {
            for (_,bone) in skeleton_node.joined.bones.iter() {
                let location = match Location::from_collada(&bone.location) {
                    Ok( loc ) => loc,
                    Err( _ ) => panic!("aaa"),//return Err( Error::SkeletonDifferentSizes( skeleton_node.name.clone() )),
                };

                println!("{}: {:?} {:?}", bone.name, location.position, location.rotation);
            }
        }

        println!("GEOMETRYES");

        for (_,geometry_node) in skene.geometries.iter() {
            println!("{}",geometry_node.joined.id);

            let location = match Location::from_collada(&geometry_node.location) {
                Ok( loc ) => loc,
                Err( _ ) => panic!("aaa"),//return Err( Error::SkeletonDifferentSizes( skeleton_node.name.clone() )),
            };

            println!("{:?} {:?}", location.position, location.rotation);
        }
        */
        /*
        for (scene_name, scene) in document.scenes.iter(){
            let (skeleton,zero_frame)=read_skeleton(scene)?;
            let (joints_geometry, bones_geometry)=skeleton.build_geometry();

            let virtual_meshes=VirtualModel::generate_virtual_meshes(&document, scene)?;//TODO: generate model_name for scene

            let model=Model::build(&document, &virtual_meshes, skeleton, zero_frame, model_name.clone(), object, to_render_tx)?;

            object.add_model(model);
        }
        */

        Ok(())
    }

    pub fn from_virtual(
        document:&collada::Document,
        virtual_model:&VirtualModel,
        object:&Storage,
        to_render_tx:&render::Sender
    ) -> Result<Arc<Self>,Error> {
        let (skeleton, zero_frame, joints_geometry, bones_geometry) = Skeleton::from_virtual(&virtual_model.skeleton, &virtual_model.location)?;

        let model=Model::new(
            virtual_model.get_name().clone(),
            String::new(),

            skeleton,
            zero_frame,

            object
        )?;

        for (_,virtual_mesh) in virtual_model.meshes.iter(){
            let mesh=Mesh::build(virtual_mesh,object,to_render_tx)?;

            model.add_mesh(mesh);
        }

        to_render_tx.send( render::Task::LoadSkeleton(model.clone(), joints_geometry, bones_geometry) )?;

        Ok(model)
    }

    pub fn prepare_skeleton(&self, frame:&mut render::Frame) -> Result<(),render::Error> {
        let mut skeleton_guard=self.skeleton.write().unwrap();
        skeleton_guard.calculate_matrices(frame)
    }

    pub fn render(&self, frame:&mut render::Frame) -> Result<(),render::Error> {
        let skeleton_guard=self.skeleton.read().unwrap();

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

    pub fn render_skeleton(&self, frame:&mut render::Frame) -> Result<(),render::Error> {
        let skeleton_guard=self.skeleton.read().unwrap();

        {
            let attrib=self.attrib.read().unwrap();

            if !attrib.include || !attrib.display {
                return Ok(());
            }
        }

        skeleton_guard.render(frame)?;

        Ok(())
    }
}
