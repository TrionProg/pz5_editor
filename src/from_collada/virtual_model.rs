use std;
use pz5;
use collada;
use std::path::Path;

use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::sync::Arc;
use std::rc::{Rc,Weak};
use std::cell::RefCell;

use collada::Document;
use collada::Scene;

use super::VirtualMesh;
use super::VirtualLOD;
use super::VirtualInstance;
use super::VirtualSkeleton;

use super::Error;
use super::location::pos3d_from_collada;
use super::location::quaternion_from_collada;

use location::Location;


pub struct VirtualModel<'a> {
    pub best_name:Option<String>,
    pub location:Location,
    pub skeleton:Option<VirtualSkeleton<'a>>,
    pub meshes:HashMap<String,VirtualMesh<'a>>,
    pub animations:Vec< Vec<&'a Arc<collada::Animation>> >,

    pub instances:Vec< Weak<VirtualInstance<'a>> >,
    //location or store
}

impl<'a> VirtualModel<'a>{
    pub fn parse_collada(file_name:&Path) -> Result<Document,Error>{
        match collada::Document::parse(file_name){
            Ok( d ) => Ok(d),
            Err(e) => Err(Error::ColladaError(e)),
        }
    }

    pub fn new(skeleton:Option<&'a Arc<collada::Skeleton>>, best_name:Option<String>, location:Location) -> Result<Self,Error> {
        let virtual_skeleton=match skeleton {
            Some( skeleton ) => Some( VirtualSkeleton::new(&location,skeleton)? ),
            None => None,
        };

        let virtual_model=VirtualModel{
            best_name:best_name,
            location:location,
            skeleton:virtual_skeleton,
            meshes:HashMap::new(),
            animations:Vec::new(),

            instances:Vec::new(),
        };

        Ok( virtual_model )
    }

    pub fn generate_virtual_models(document:&'a Document, scene:&'a Scene) -> Result< HashMap<String,VirtualModel<'a>>,Error>{
        let mut virtual_models=HashMap::new();

        for (_, geometry_node) in scene.geometries.iter() {
            let geometry=&geometry_node.joined;

            let (node_name, distance)=match geometry_node.name.find("_d_"){
                Some( pos ) => {
                    let (node_name, wast_and_distance)=geometry_node.name.split_at(pos);
                    let (wast,distance_str)=wast_and_distance.split_at("_d_".len());

                    let distance=match distance_str.parse::<f32>(){
                        Ok(d) => d,
                        Err(_) => return Err(Error::StringParseError( format!("Can not parse {} as f32",distance_str) )),
                    };

                    (String::from(node_name), distance)
                },
                None =>
                    (geometry_node.name.clone(),0.0),
            };

            println!("{} {}",&node_name, &distance);

            //get or crate virtual_model(skeleton)

            let (virtual_model, location)=match geometry_node.controller {
                collada::Controller::Skin( ref skin ) => {
                    if !virtual_models.contains_key(&skin.skeleton_id) {
                        let skeleton = match document.skeletons.get( &skin.skeleton_id ) {
                            Some( skeleton ) => skeleton,
                            None => return Err(Error::NoSkeleton( skin.skeleton_id.clone() )),
                        };

                        let skeleton_location = match Location::from_collada(&skeleton.location) {
                            Ok ( location ) => location,
                            Err( _ ) => {
                                return Err( Error::Other( format!("Skeleton \"{}\" has different scales",skeleton.id) ));
                            },
                        };

                        virtual_models.insert(skin.skeleton_id.clone(), VirtualModel::new(Some(skeleton),None,skeleton_location)? );
                    }

                    let virtual_model=virtual_models.get_mut(&skin.skeleton_id).unwrap();

                    if virtual_model.best_name.is_none() {
                        virtual_model.best_name=Some( skin.skeleton_name.clone() );
                    }

                    let location = match Location::from_collada(&skin.bind_location) {
                        Ok ( location ) => location,
                        Err( _ ) => {
                            return Err( Error::Other( format!("Skin \"{}\" has different scales",skin.id) ));
                        },
                    };

                    (virtual_model, location)
                },
                collada::Controller::Bone( ref bone ) => {
                    if !virtual_models.contains_key(&bone.skeleton_id) {
                        let skeleton = match document.skeletons.get( &bone.skeleton_id ) {
                            Some( skeleton ) => skeleton,
                            None => return Err(Error::NoSkeleton( bone.skeleton_id.clone() )),
                        };

                        let skeleton_location = match Location::from_collada(&skeleton.location) {
                            Ok ( location ) => location,
                            Err( _ ) => {
                                return Err( Error::Other( format!("Skeleton \"{}\" has different scales",skeleton.id) ));
                            },
                        };

                        virtual_models.insert(bone.skeleton_id.clone(), VirtualModel::new(Some(skeleton),None,skeleton_location)? );
                    }

                    let virtual_model=virtual_models.get_mut(&bone.skeleton_id).unwrap();

                    let location = match Location::from_collada(&geometry_node.location) {
                        Ok ( location ) => location,
                        Err( _ ) => {
                            return Err( Error::Other( format!("Geometry \"{}\" has different scales",geometry_node.name) ));
                        },
                    };

                    (virtual_model, location)
                },
                collada::Controller::Model => {
                    if !virtual_models.contains_key(&node_name) {
                        let model_location = match Location::from_collada(&geometry_node.location) {
                            Ok ( location ) => location,
                            Err( _ ) => {
                                return Err( Error::Other( format!("Geometry \"{}\" has different scales",geometry_node.name) ));
                            },
                        };

                        virtual_models.insert(node_name.clone(), VirtualModel::new(None,Some( node_name.clone()),model_location)? );
                    }

                    let virtual_model=virtual_models.get_mut(&node_name).unwrap();

                    let location = match Location::from_collada(&geometry_node.location) {
                        Ok ( location ) => location,
                        Err( _ ) => {
                            return Err( Error::Other( format!("Geometry \"{}\" has different scales",geometry_node.name) ));
                        },
                    };

                    (virtual_model, location)
                },
            };

            let mesh_location=location-virtual_model.location;

            //insert meshes

            for (i,mesh) in geometry.meshes.iter().enumerate(){
                let mesh_name=if geometry.meshes.len()==1 {
                    node_name.clone()
                }else{
                    match mesh.material{
                        Some( ref material_id ) =>
                            format!("{}_{}",node_name,material_id),
                        None =>
                            format!("{} #{}",node_name, i),
                    }
                };

                let virtual_lod=VirtualLOD::construct(&mesh, distance)?;

                match virtual_model.meshes.entry(mesh_name.clone()){
                    Entry::Vacant(entry) => {
                        let geometry_type=virtual_lod.geometry_type;
                        let vertex_format=virtual_lod.geometry.vertex_format.clone();

                        let mut lods=Vec::with_capacity(1);
                        lods.push(virtual_lod);//TODO:check lods has same location

                        entry.insert(
                            VirtualMesh{
                                name:mesh_name,
                                vertex_format:vertex_format,
                                location:mesh_location.clone(),

                                lods:lods,
                                geometry_type:geometry_type,
                                controller:geometry_node.controller.clone(),
                            }
                        );
                    },
                    Entry::Occupied(mut entry) => {
                        let mesh=entry.get_mut();
                        if mesh_location != mesh.location {
                            return Err(Error::MeshLODLocationsMismatch( mesh.name.clone(), geometry_node.name.clone() ));
                        }

                        mesh.lods.push(virtual_lod);
                    },
                }
            }
        }

        for (_,animation) in document.animations.iter() {
            match virtual_models.get_mut( &animation.skeleton_id ){
                Some( virtual_model ) => {
                    if virtual_model.animations.len()==0 {
                        virtual_model.animations.push( Vec::new() );
                    }

                    virtual_model.animations[0].push( animation );
                },
                None => {},
            }
        }

        Ok( virtual_models )
    }

    pub fn get_name(&self) -> &String {
        match self.best_name {
            Some( ref name ) => name,
            None => unreachable!(),
        }
    }

    pub fn check_and_sort_virtual_models(virtual_models:&mut HashMap<String,VirtualModel<'a>>) -> Result<(),Error> {
        for (_,virtual_model) in virtual_models.iter_mut() {
            for (_,virtual_mesh) in virtual_model.meshes.iter_mut(){
                virtual_mesh.lods.sort_by(|lod1,lod2| lod1.distance.partial_cmp(&lod2.distance).unwrap());
                println!("H:{}",virtual_mesh.lods.len());
            }

            for (_,virtual_mesh) in virtual_model.meshes.iter(){
                virtual_mesh.check()?;
            }

            if virtual_model.best_name.is_none() {
                virtual_model.best_name=match virtual_model.skeleton {
                    Some( ref skeleton ) => Some(skeleton.collada_skeleton.id.clone()),
                    None => unreachable!(),
                }
            }
        }

        Ok(())
    }

    pub fn separate_to_models_and_instances(mut virtual_models_instances:HashMap<String,VirtualModel<'a>>) ->
        Result<(Vec< Rc<RefCell<VirtualModel<'a>>> >, Vec< Rc<VirtualInstance<'a>> >), Error>
    {
        let mut virtual_models:Vec< Rc<RefCell<VirtualModel<'a>>> > = Vec::new();
        let mut virtual_instances:Vec< Rc<VirtualInstance<'a>> > = Vec::with_capacity(virtual_models.len());

        'instance_loop: for (_,mut virtual_model_instance) in virtual_models_instances.drain() {
            for virtual_model_rc in virtual_models.iter() {
                let mut virtual_model=virtual_model_rc.borrow_mut();
                if virtual_model.eq(&virtual_model_instance) {
                    if virtual_model.skeleton.is_some() {
                        virtual_model.animations.append( &mut virtual_model_instance.animations );
                    }

                    //rename model
                    if virtual_model_instance.get_name() < virtual_model.get_name() {
                        virtual_model.best_name=virtual_model_instance.best_name.clone();
                    }

                    //rename meshes
                    for (_,virtual_mesh1) in virtual_model.meshes.iter_mut() {
                        for (_,virtual_mesh2) in virtual_model_instance.meshes.iter() {
                            if virtual_mesh1==virtual_mesh2 {
                                if virtual_mesh2.name < virtual_mesh1.name {
                                    virtual_mesh1.name=virtual_mesh2.name.clone();
                                }
                            }
                        }
                    }

                    let virtual_instance=Rc::new( VirtualInstance::new(
                        virtual_model_rc,
                        virtual_model_instance.best_name.unwrap(),
                        virtual_model_instance.location
                    ) );
                    virtual_model.instances.push( Rc::downgrade(&virtual_instance) );
                    virtual_instances.push( virtual_instance );

                    continue 'instance_loop;
                }
            }

            let instance_name=match virtual_model_instance.best_name {
                Some( ref best_name ) => best_name.clone(),
                None => unreachable!(),
            };

            let instance_location=virtual_model_instance.location.clone();

            let virtual_model=Rc::new(RefCell::new( virtual_model_instance ));
            let virtual_instance=Rc::new( VirtualInstance::new(
                &virtual_model,
                instance_name,
                instance_location
            ) );
            virtual_model.borrow_mut().instances.push( Rc::downgrade(&virtual_instance) );
            virtual_instances.push( virtual_instance );
            virtual_models.push( virtual_model );
        }

        Ok( (virtual_models,virtual_instances) )
    }
}

impl<'a> PartialEq for VirtualModel<'a> {
    fn eq(&self, other:&Self) -> bool {
        let skeletons_eq = match self.skeleton {
            Some( ref skeleton1 ) => {
                match other.skeleton {
                    Some( ref skeleton2 ) => skeleton1 == skeleton2,
                    None => false,
                }
            },
            None => {
                match other.skeleton {
                    Some( _ ) => false,
                    None => true,
                }
            }
        };

        //println!("0");

        if !skeletons_eq {
            //println!("1");
            return false;
        }

        if self.meshes.len() != other.meshes.len() {
            //println!("2");
            return false;
        }

        'mesh_loop: for (_,mesh1) in self.meshes.iter() {
            for (_,mesh2) in other.meshes.iter() {
                if mesh1==mesh2 {
                    continue 'mesh_loop;
                }
            }

            return false;//no matching mesh have been found
        }

        true
    }

    fn ne(&self, other:&Self) -> bool {
        !self.eq(other)
    }
}
