use std;
use glium;

use std::sync::Arc;
use std::sync::{Mutex,RwLock};

use std::path::Path;
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::sync::mpsc;

use Error;
use RenderSender;
use GrowableSlab;

use super::LOD;
use super::Mesh;
use super::Model;

//use ObjectFrame;

pub struct Object{
    pub models: RwLock< HashMap<String, Arc<Model> > >,
    pub slab_lods: GrowableSlab<LOD>,
    pub slab_meshes: GrowableSlab<Mesh>,
    pub slab_models: GrowableSlab<Model>,
    pub is_gui: bool,
}

impl Object{
    pub fn empty(is_gui:bool) -> Self{
        Object{
            models: RwLock::new( HashMap::new() ),
            slab_lods: GrowableSlab::with_capacity(60),
            slab_meshes: GrowableSlab::with_capacity(30),
            slab_models: GrowableSlab::with_capacity(2),
            is_gui: is_gui,
        }
    }


    pub fn include_collada_model(&self, file_name:&Path, to_render_tx:&RenderSender) -> Result<(),Error> {
        Model::load_from_collada(file_name,self,to_render_tx)
    }


    pub fn add_lod_to_list(&self,lod:LOD) -> Result< Arc<LOD>, Error >{
        let ref_lod=self.slab_lods.insert(lod);

        Ok(ref_lod)
    }

    pub fn add_mesh_to_list(&self,mut mesh:Mesh) -> Result< Arc<Mesh>, Error >{
        let ref_mesh=self.slab_meshes.insert(mesh);

        Ok(ref_mesh)
    }

    pub fn add_model_to_list(&self,mut model:Model) -> Result< Arc<Model>, Error >{
        let ref_model=self.slab_models.insert(model);

        Ok(ref_model)
    }

    pub fn add_model(&self,model:Arc<Model>){
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
                    let name=format!("{}.{}",base_name,cnt);
                    model.attrib.write().unwrap().name=name.clone();
                }
            }
        }
    }
    /*

    pub fn include_collada_model(&mut self, file_name:&Path) -> Result<(),Error> {
        let model=Model::include_collada_model(file_name, self.model_id, &self.render)?;

        self.models.insert(self.model_id, Rc::new(model));
        self.model_id+=1;

        Ok(())
    }

    pub fn render(&self, frame:&mut ObjectFrame) -> Result<(),glium::DrawError>{
        for (_,model) in self.models.iter(){
            model.render(frame)?;
        }

        Ok(())
    }
    */
}
