use std;
use glium;

use std::sync::Arc;
use std::sync::{Mutex,RwLock};

use std::path::Path;
use std::collections::HashMap;
use std::collections::hash_map::Entry;

use Error;

use super::LOD;
use super::Mesh;
use super::Model;

//use ObjectFrame;

pub struct Object{
    pub models: RwLock< HashMap<String, Arc<Model> > >,
    pub list_lods: RwLock< Vec<Option<Arc<LOD>>> >,
    pub list_meshes: RwLock< Vec<Option<Arc<Mesh>>> >,
    pub list_models: RwLock< Vec<Option<Arc<Model>>> >,
    pub is_gui: bool,
}

impl Object{
    pub fn empty(is_gui:bool) -> Self{
        Object{
            models: RwLock::new( HashMap::new() ),
            list_lods: RwLock::new( Vec::new() ),
            list_meshes: RwLock::new( Vec::new() ),
            list_models: RwLock::new( Vec::new() ),
            is_gui: is_gui,
        }
    }


    pub fn include_collada_model(&self, file_name:&Path) -> Result<(),Error> {
        Model::load_from_collada(file_name,self)
    }


    pub fn add_lod_to_list(&self,mut lod:LOD) -> Result< Arc<LOD>, Error >{
        let mut list_lods_guard=self.list_lods.write().unwrap();

        lod.id=list_lods_guard.len();
        let ref_lod=Arc::new( lod );

        list_lods_guard.push(Some(ref_lod.clone()));

        Ok(ref_lod)
    }

    pub fn add_mesh_to_list(&self,mut mesh:Mesh) -> Result< Arc<Mesh>, Error >{
        let mut list_meshes_guard=self.list_meshes.write().unwrap();

        mesh.id=list_meshes_guard.len();
        let ref_mesh=Arc::new( mesh );

        list_meshes_guard.push(Some(ref_mesh.clone()));

        Ok(ref_mesh)
    }

    pub fn add_model_to_list(&self,mut model:Model) -> Result< Arc<Model>, Error >{
        let mut list_models_guard=self.list_models.write().unwrap();

        model.id=list_models_guard.len();
        let ref_model=Arc::new( model );

        list_models_guard.push(Some(ref_model.clone()));

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
