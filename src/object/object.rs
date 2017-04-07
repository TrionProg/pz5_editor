use std;
use glium;

use std::sync::Arc;
use std::sync::{Mutex,RwLock};

use std::path::Path;
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::sync::mpsc;

use object_pool::multithreaded_growable::Pool as MTPool;
use object_pool::multithreaded_growable::{ID,Slot};

use ProcessError;
use RenderSender;
use RenderError;
use RenderFrame;

use super::LOD;
use super::Mesh;
use super::Model;

pub struct Object{
    pub models: RwLock< HashMap<String, Arc<Model> > >,
    pub pool_lods: MTPool<LOD>,
    pub pool_meshes: MTPool<Mesh>,
    pub pool_models: MTPool<Model>,
    pub is_gui: bool,
}

impl Object{
    pub fn empty(is_gui:bool) -> Self{
        Object{
            models: RwLock::new( HashMap::new() ),
            pool_lods: MTPool::new(),
            pool_meshes: MTPool::new(),
            pool_models: MTPool::new(),
            is_gui: is_gui,
        }
    }


    pub fn include_collada_model(&self, file_name:&Path, to_render_tx:&RenderSender) -> Result<(),ProcessError> {
        Model::load_from_collada(file_name,self,to_render_tx)
    }


    pub fn add_lod_to_pool(&self, lod:LOD) -> Result< Arc<LOD>, ProcessError >{
        let ref_lod=self.pool_lods.insert(lod);

        Ok(ref_lod)
    }

    pub fn add_mesh_to_pool(&self, mut mesh:Mesh) -> Result< Arc<Mesh>, ProcessError >{
        let ref_mesh=self.pool_meshes.insert(mesh);

        Ok(ref_mesh)
    }

    pub fn add_model_to_pool(&self, mut model:Model) -> Result< Arc<Model>, ProcessError >{
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

    //TODO:add removemodel method

    pub fn render(&self, frame:&mut RenderFrame) -> Result<(),RenderError> {
        let models_guard=self.models.read().unwrap();

        for (_,model) in models_guard.iter() {
            model.prepare_skeleton(frame)?;
            model.render(frame)?;
        }

        Ok(())
    }

    pub fn render_skeletons(&self, frame:&mut RenderFrame) -> Result<(),RenderError> {
        let models_guard=self.models.read().unwrap();

        for (_,model) in models_guard.iter() {
            model.render_skeleton(frame)?;
        }

        Ok(())
    }
}
