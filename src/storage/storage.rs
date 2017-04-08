use std;
use object_pool;
use render;
use object;

use std::path::Path;
use std::rc::Rc;
use std::sync::Arc;
use std::collections::HashMap;
use object_pool::growable::{Pool,ID,Slot};

use pz5::GeometryType;
use pz5::Pz5Geometry;

use RenderError;
use Window;
use super::Grid;
use super::GridShader;
use super::ModelShader;
use super::VBO;
use super::Geometry;
use super::SkeletonShader;
use super::Skeleton;

pub struct Storage{
    pub model_shaders:HashMap<String,Rc<ModelShader>>,
    pub grid_shader:GridShader,
    pub skeleton_shader:SkeletonShader,
    pub grid:Grid,
    pub geometries:Pool<Geometry,Geometry>,
    pub skeletons:Pool<Skeleton,Skeleton>,
    //textures:
}

impl Storage{
    pub fn new(window:&Window) -> Result<Self,RenderError> {
        let model_shaders = ModelShader::generate_model_shaders(window)?;

        let grid_shader=GridShader::new(window)?;
        let skeleton_shader=SkeletonShader::new(window)?;
        let grid=Grid::new(10.0, window)?;

        let storage=Storage{
            model_shaders:model_shaders,
            grid_shader:grid_shader,
            skeleton_shader:skeleton_shader,
            grid:grid,
            geometries:Pool::new(),
            skeletons:Pool::new(),
        };

        Ok(storage)
    }

    pub fn load_geometry(&mut self,
        lod:Arc<object::LOD>,

        geometry:Pz5Geometry,
        geometry_type:GeometryType,
        vertex_format:String,

        window:&Window,
    ) -> Result<(),RenderError> {
        let mut geometry_id_guard=lod.geometry_id.lock().unwrap();

        match *geometry_id_guard {
            Some( ref geometry_id ) => {self.geometries.remove(*geometry_id);},
            None => {},
        }

        *geometry_id_guard=None;

        let shader=match self.model_shaders.get(&vertex_format) {
            Some( shader ) => shader.clone(),
            None => return Err( RenderError::NoShaderProgram(vertex_format) ),
        };

        let mut geometry=Geometry::new(geometry,geometry_type,vertex_format,shader,window)?;

        let inserted_geometry=self.geometries.insert(geometry);

        *geometry_id_guard=Some(inserted_geometry.id);

        Ok(())
    }

    pub fn load_skeleton(&mut self,
        model:Arc<object::Model>,
        joints_geometry:Vec<super::skeleton::Vertex>,
        bones_geometry:Vec<super::skeleton::Vertex>,
        window:&Window,
    ) -> Result<(),RenderError> {
        let mut skeleton_id_guard=model.skeleton.write().unwrap();

        match skeleton_id_guard.skeleton_id {
            Some( ref skeleton_id ) => {self.skeletons.remove(*skeleton_id);},
            None => {},
        }

        skeleton_id_guard.skeleton_id=None;

        let mut skeleton=Skeleton::new(joints_geometry, bones_geometry, window)?;

        let inserted_skeleton=self.skeletons.insert(skeleton);

        skeleton_id_guard.skeleton_id=Some(inserted_skeleton.id);

        Ok(())
    }
}
