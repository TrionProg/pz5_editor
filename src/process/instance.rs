use std;
use cgmath;
use location::*;
use render;

use object_pool::growable::{ID,Slot};
use std::sync::Arc;
use std::sync::{Mutex,RwLock};

use super::Error;
use super::Model;

pub struct InstanceAttrib {
    pub name:String,
    pub include:bool,
    pub display:bool,
}

pub struct Instance {
    pub model:Arc<Model>,
    pub attrib:RwLock<InstanceAttrib>,
    pub skeleton:RwLock<SkeletonOfInstance>,
}

pub struct SkeletonOfInstance {
    pub bones_locations:Vec<Location>,
    pub bones_matrices:Vec<Matrix4>,
    pub skeleton_id:Option<ID>,
    pub animated:bool,
}

impl Instance {
    pub fn new(name:String, model:Arc<Model>, location:&Location, to_render_tx:&render::Sender) -> Result<Arc<Self>,Error> {
        let mut skeleton=SkeletonOfInstance::new(&model,location);
        let bones_count=skeleton.bones_matrices.len();

        let instance=Arc::new(Instance{
            model:model,
            attrib:RwLock::new(InstanceAttrib{
                name:name,
                include:true,
                display:true,
            }),
            skeleton:RwLock::new(skeleton),
        });

        to_render_tx.send(render::Task::LoadSkeletonOfInstance( instance.clone(), bones_count ))?;

        Ok( instance )
    }

    pub fn display(&self) -> bool {
        let attrib_guard=self.attrib.read().unwrap();

        attrib_guard.include && attrib_guard.display
    }

    fn calculate_matrices(&self, frame:&mut render::Frame) -> Result<(),render::Error> {
        let mut instance_skeleton_guard=self.skeleton.write().unwrap();

        instance_skeleton_guard.calculate_matrices(frame)
    }

    pub fn render(&self, frame:&mut render::Frame) -> Result<(),render::Error> {
        if !self.display() {
            return Ok(());
        }

        self.calculate_matrices(frame)?;
        let instance_skeleton_guard=self.skeleton.read().unwrap();

        //self.model.render(, &instance_skeleton_guard)?;

        Ok(())
    }

    pub fn render_skeleton(&self, frame:&mut render::Frame) -> Result<(),render::Error> {
        if !self.display() {
            return Ok(());
        }

        self.calculate_matrices(frame)?;
        let skeleton_of_instance_guard=self.skeleton.read().unwrap();

        self.model.render_skeleton(frame, &skeleton_of_instance_guard)
    }
}

impl SkeletonOfInstance {
    pub fn new(model:&Model,root_location:&Location) -> Self {
        use cgmath::SquareMatrix;

        let skeleton=model.skeleton.read().unwrap();
        let zero_frame=model.zero_frame.read().unwrap();

        let mut bones_locations=zero_frame.bones_locations.clone();
        bones_locations[0]=*root_location;
        let bones_matrices=vec!(Matrix4::identity();zero_frame.bones_locations.len());

        SkeletonOfInstance {
            bones_locations:bones_locations,
            bones_matrices:bones_matrices,
            skeleton_id:None,
            animated:true,
        }
    }


    pub fn calculate_matrices(&mut self, frame:&mut render::Frame) -> Result<(),render::Error> {
        if !self.animated {
            return Ok(());
        }

        self.animated=false;

        use cgmath::SquareMatrix;

        for i in 0..self.bones_locations.len() {
            self.bones_matrices[i]=Matrix4::identity();
        }

        let skeleton=self.get_skeleton_of_instance_from_storage(frame)?;

        skeleton.load_bones( &self.bones_matrices[..] );

        Ok(())
    }
        /*

        for i in 0..self.bones.len() {
            use cgmath::SquareMatrix;
            /*
            println!("{:?} {:?}",self.bones[i].location.position,self.bones[i].location.rotation);
            use cgmath::SquareMatrix;
            use cgmath::Vector3;
            use cgmath::EuclideanSpace;

            let m=
            Matrix4::from_translation(self.bones[i].location.position.to_vec())*
            Matrix4::from(self.bones[i].location.rotation)*
                    //Matrix4::from_translation(self.bones[i].location.position.to_vec())*
                    Matrix4::from_scale(self.bones[i].location.scale.0);

            let m=match self.bones[i].parent_index {
                Some( parent_index ) => self.matrices[parent_index] * m,
                _ => m,
            };
            */
            use cgmath::EuclideanSpace;

            self.matrices[i]=Matrix4::from_translation( self.bones[0].location.position.to_vec() );//m;//*Matrix4::from(self.bones[i].location.rotation);
            /*
            let bone_matrix=self.bones[i].calculate_matrix(&self.matrices);
            use cgmath::SquareMatrix;
            self.matrices[i]=bone_matrix;
            */

            //println!("{} {:?}",i,self.bones[i].parent_index);
        }

        */

        /*
        //self.bones[1].location.position.z=1.0;

        println!("{:?} {:?}",self.bones[11].location.position,self.bones[11].location.rotation);
        println!("{:?} {:?}",self.bones[3].location.position,self.bones[3].location.rotation);
        println!("{:?} {:?}",self.bones[2].location.position,self.bones[2].location.rotation);
        println!("{:?}",self.matrices[11]);
        println!("{:?}",self.matrices[3]);
        println!("{:?}",self.matrices[2]);
        use cgmath::Vector4;
        println!("{:?}",self.matrices[11]*Vector4::new(0.0,0.0,0.0,1.0));
        println!("{:?}",self.matrices[3]*Vector4::new(0.0,0.0,0.0,1.0));
        println!("{:?}",self.matrices[9]*Vector4::new(0.0,0.0,0.0,1.0));
        println!("{:?}",self.matrices[10]*Vector4::new(0.0,0.0,0.0,1.0));
        println!("{:?}",self.matrices[2]*Vector4::new(0.0,0.0,0.0,1.0));
        */


        let skeleton=self.get_skeleton_from_storage(frame)?;

        skeleton.load_bones( &self.matrices[..] );

        Ok(())
    }
    */
    /*
    fn get_skeleton_from_storage<'a>(&self, frame:&'a render::Frame ) -> Result<&'a render::Skeleton,render::Error> {
        let skeleton_id = match self.skeleton_id {
            Some( ref skeleton_id ) => *skeleton_id,
            None => return Err(render::Error::NoSkeleton),
        };

        let skeleton=match frame.storage.skeletons.get(skeleton_id) {
            Some( skeleton ) => skeleton,
            None => return Err(render::Error::NoGeometryWithID(skeleton_id)),
        };

        Ok(skeleton)
    }
    */

    pub fn get_skeleton_of_instance_from_storage<'a>(&self, frame:&render::Frame<'a>) -> Result<&'a render::SkeletonOfInstance,render::Error> {
        let skeleton_of_instance_id = match self.skeleton_id {
            Some( ref skeleton_of_instance_id ) => *skeleton_of_instance_id,
            None => return Err(render::Error::NoSkeleton),
        };

        frame.storage.get_skeleton_of_instance(skeleton_of_instance_id)
    }

}
