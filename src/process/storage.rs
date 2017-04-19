use std;
use glium;
use render;

use std::sync::Arc;
use std::sync::{Mutex,RwLock};

use std::path::Path;
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::sync::mpsc;

use object_pool::multithreaded_growable::Pool as MTPool;
use object_pool::multithreaded_growable::{ID,Slot};

use super::Error;

use super::LOD;
use super::Mesh;
use super::Model;
use super::Scene;

pub struct Storage{
    pub models: RwLock< HashMap<String, Arc<Model> > >,
    pub scenes: RwLock< HashMap<String, Arc<Scene> > >,
    pub pool_lods: MTPool<LOD>,
    pub pool_meshes: MTPool<Mesh>,
    pub pool_models: MTPool<Model>,
    pub is_gui: bool,
}

impl Storage{
    pub fn empty(is_gui:bool) -> Self{
        Storage{
            models: RwLock::new( HashMap::new() ),
            scenes: RwLock::new( HashMap::new() ),
            pool_lods: MTPool::new(),
            pool_meshes: MTPool::new(),
            pool_models: MTPool::new(),
            is_gui: is_gui,
        }
    }


    pub fn include_collada_model(&self, file_name:&Path, to_render_tx:&render::Sender) -> Result<(),Error> {
        Model::load_from_collada(file_name,self,to_render_tx)
    }


    pub fn add_lod_to_pool(&self, lod:LOD) -> Result< Arc<LOD>, Error >{
        let ref_lod=self.pool_lods.insert(lod);

        Ok(ref_lod)
    }

    pub fn add_mesh_to_pool(&self, mut mesh:Mesh) -> Result< Arc<Mesh>, Error >{
        let ref_mesh=self.pool_meshes.insert(mesh);

        Ok(ref_mesh)
    }

    pub fn add_model_to_pool(&self, mut model:Model) -> Result< Arc<Model>, Error >{
        let ref_model=self.pool_models.insert(model);

        Ok(ref_model)
    }

    pub fn add_model(&self, model:Arc<Model>){
        let mut models_guard=self.models.write().unwrap();

        let mut cnt=0;
        let base_name=model.attrib.read().unwrap().name.clone();
        let mut name=base_name.clone();

        loop{
            match models_guard.entry(name.clone()) {
                Entry::Vacant(e) => {
                    e.insert(model);
                    break;
                },
                Entry::Occupied(_) => {
                    cnt+=1;
                    name=format!("{}.{}",base_name,cnt);
                    model.attrib.write().unwrap().name=name.clone();
                }
            }
        }
    }

    pub fn add_scene(&self, scene:Arc<Scene>){
        let mut scenes_guard=self.scenes.write().unwrap();

        let mut cnt=0;
        let base_name=scene.attrib.read().unwrap().name.clone();
        let mut name=base_name.clone();

        loop{
            match scenes_guard.entry(name.clone()) {
                Entry::Vacant(e) => {
                    e.insert(scene);
                    break;
                },
                Entry::Occupied(_) => {
                    cnt+=1;
                    name=format!("{}.{}",base_name,cnt);
                    scene.attrib.write().unwrap().name=name.clone();
                }
            }
        }
    }

    //TODO:add removemodel method

    pub fn render(&self, frame:&mut render::Frame) -> Result<(),render::Error> {
        let scenes_guard=self.scenes.read().unwrap();

        for (_,scene) in scenes_guard.iter() {
            scene.render(frame)?;
        }

        Ok(())
    }

    pub fn render_skeletons(&self, frame:&mut render::Frame) -> Result<(),render::Error> {
        let scenes_guard=self.scenes.read().unwrap();

        for (_,scene) in scenes_guard.iter() {
            scene.render_skeletons(frame)?;
        }

        Ok(())
    }
}
