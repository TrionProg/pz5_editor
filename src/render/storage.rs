use std;
use object_pool;
use process;

use std::path::Path;
use std::rc::Rc;
use std::sync::Arc;
use std::collections::HashMap;
use object_pool::growable::{Pool,ID,Slot};

use pz5::GeometryType;
use pz5::Pz5Geometry;

use super::Error;
use super::Window;
use super::Grid;
use super::GridShader;
use super::ModelShader;
use super::VBO;
use super::Geometry;
use super::SkeletonShader;
use super::SkeletonOfInstance;
use super::GeometryOfSkeleton;

pub struct Storage{
    pub model_shaders:HashMap<String,Rc<ModelShader>>,
    pub grid_shader:GridShader,
    pub skeleton_shader:SkeletonShader,
    pub grid:Grid,
    pub geometries:Pool<Geometry,Geometry>,
    pub skeletons_of_instances:Pool<SkeletonOfInstance,SkeletonOfInstance>,
    pub geometries_of_skeletons:Pool<GeometryOfSkeleton,GeometryOfSkeleton>,
    //textures:
}

impl Storage{
    pub fn new(window:&Window) -> Result<Self,Error> {
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
            skeletons_of_instances:Pool::new(),
            geometries_of_skeletons:Pool::new(),
        };

        Ok(storage)
    }

    pub fn load_geometry(&mut self,
        lod:Arc<process::LOD>,

        geometry:Pz5Geometry,
        geometry_type:GeometryType,
        vertex_format:String,

        window:&Window,
    ) -> Result<(),Error> {
        let mut geometry_id_guard=lod.geometry_id.lock().unwrap();

        match *geometry_id_guard {
            Some( ref geometry_id ) => {self.geometries.remove(*geometry_id);},
            None => {},
        }

        *geometry_id_guard=None;

        let shader=match self.model_shaders.get(&vertex_format) {
            Some( shader ) => shader.clone(),
            None => return Err( Error::NoShaderProgram(vertex_format) ),
        };

        let mut geometry=Geometry::new(geometry,geometry_type,vertex_format,shader,window)?;

        let inserted_geometry=self.geometries.insert(geometry);

        *geometry_id_guard=Some(inserted_geometry.id);

        Ok(())
    }

    pub fn load_skeleton_of_instance(&mut self,
        instance:Arc<process::Instance>,
        bones_count:usize,
        window:&Window,
    ) -> Result<(),Error> {
        let mut skeleton_guard=instance.skeleton.write().unwrap();

        match skeleton_guard.skeleton_id {
            Some( ref skeleton_id ) => {self.skeletons_of_instances.remove(*skeleton_id);},
            None => {},
        }

        skeleton_guard.skeleton_id=None;//Skeleton::new may return Error

        let mut skeleton=SkeletonOfInstance::new(bones_count, window)?;

        let inserted_skeleton=self.skeletons_of_instances.insert(skeleton);

        skeleton_guard.skeleton_id=Some(inserted_skeleton.id);

        Ok(())
    }

    pub fn load_geometry_of_skeleton(&mut self,
        model:Arc<process::Model>,
        joints_geometry:Vec<super::skeleton::Vertex>,
        bones_geometry:Vec<super::skeleton::Vertex>,
        window:&Window,
    ) -> Result<(),Error> {
        let mut skeleton_guard=model.skeleton.write().unwrap();

        match skeleton_guard.geometry_of_skeleton_id {
            Some( ref skeleton_id ) => {self.geometries_of_skeletons.remove(*skeleton_id);},
            None => {},
        }

        skeleton_guard.geometry_of_skeleton_id=None;//Skeleton::new may return Error

        let mut skeleton=GeometryOfSkeleton::new(joints_geometry, bones_geometry, window)?;

        let inserted_skeleton=self.geometries_of_skeletons.insert(skeleton);

        skeleton_guard.geometry_of_skeleton_id=Some(inserted_skeleton.id);

        Ok(())
    }

    pub fn get_skeleton_of_instance(&self, skeleton_id:ID) -> Result<&SkeletonOfInstance,Error> {
        match self.skeletons_of_instances.get(skeleton_id) {
            Some( skeleton ) => Ok(skeleton),
            None => return Err(Error::NoSkeletonOfInstanceWithID(skeleton_id)),
        }
    }

    pub fn get_geometry_of_skeleton(&self, skeleton_geometry_id:ID) -> Result<&GeometryOfSkeleton,Error> {
        match self.geometries_of_skeletons.get(skeleton_geometry_id) {
            Some( skeleton_geometry ) => Ok(skeleton_geometry),
            None => return Err(Error::NoGeometryOfSkeletonWithID(skeleton_geometry_id)),
        }
    }
}
