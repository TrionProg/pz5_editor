use std;
use cgmath;
use location::*;
use render;

use object_pool::growable::{ID,Slot};
use std::sync::Arc;
use std::sync::{Mutex,RwLock};
use std::time::Instant as TimeInstant;

use super::Error;
use super::Model;
use super::Animation;

pub struct InstanceAttrib {
    pub name:String,
    pub include:bool,
    pub display:bool,
}

pub struct Instance {
    pub model:Arc<Model>,
    pub attrib:RwLock<InstanceAttrib>,
    pub skeleton:RwLock<SkeletonOfInstance>,
    pub playing_animation:Mutex<Option<PlayingAnimation>>,
}

pub struct SkeletonOfInstance {
    pub bones_locations:Vec<Location>,
    pub bones_matrices:Vec<Matrix4>,
    pub skeleton_id:Option<ID>,
    pub animated:bool,
}

pub struct PlayingAnimation{
    pub animation:Arc<Animation>,
    pub start_time:TimeInstant,
    pub cyclic:bool,
    pub animating_bones_count:usize,
    pub bones_tracks:Vec<Option<usize>>,
}

impl Instance {
    pub fn new(name:String, model:Arc<Model>, location:&Location, to_render_tx:&render::Sender) -> Result<Arc<Self>,Error> {
        let mut skeleton=SkeletonOfInstance::new(&model,location);
        let bones_count=skeleton.bones_matrices.len();

        println!("AAA:{:?}", location.rotation);

        let instance=Arc::new(Instance{
            model:model,
            attrib:RwLock::new(InstanceAttrib{
                name:name,
                include:true,
                display:true,
            }),
            skeleton:RwLock::new(skeleton),
            playing_animation:Mutex::new(None),
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

    pub fn animate(&self, current_time:&TimeInstant) {
        if !self.display() {
            return;
        }

        let mut playing_animation_guard=self.playing_animation.lock().unwrap();

        let animation_finished=match *playing_animation_guard {
            Some( ref mut playing_animation ) => {
                let mut skeleton_guard=self.skeleton.write().unwrap();

                playing_animation.animate(&mut skeleton_guard, current_time)
            },
            None => false,
        };

        if animation_finished {
            *playing_animation_guard=None;
        }
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

impl PlayingAnimation {
    pub fn new(animation:&Arc<Animation>, start_time:&TimeInstant, cyclic:bool) -> Self {
        let animation_bones_tracks_guard=animation.bones_tracks.read().unwrap();
        let animating_bones_count=animation_bones_tracks_guard.len();

        let bones_tracks=vec!(Some(0);animating_bones_count);

        PlayingAnimation {
            animation:animation.clone(),
            start_time:start_time.clone(),
            cyclic:cyclic,
            animating_bones_count:animating_bones_count,
            bones_tracks:bones_tracks,
        }
    }

    pub fn animate(&mut self, skeleton:&mut SkeletonOfInstance, current_time:&TimeInstant) -> bool {
        let animation_bones_tracks_guard=self.animation.bones_tracks.read().unwrap();

        let animation_duration=current_time.duration_since(self.start_time);
        let current_time=animation_duration.as_secs() as f32 + (animation_duration.subsec_nanos()/1000_000) as f32 / 1000.0;

        'track_loop: for (play_anim_track,anim_track) in self.bones_tracks.iter_mut().zip( animation_bones_tracks_guard.iter() ) {
            let last_frame = match *play_anim_track {
                Some( last_frame ) => last_frame,
                None => continue,
            };

            for i in last_frame+1..anim_track.keyframes.len() {
                if anim_track.keyframes[i].time >= current_time {
                    let prev_frame_index=i-1;
                    let prev_frame=&anim_track.keyframes[prev_frame_index];
                    let next_frame=&anim_track.keyframes[i];

                    let k=(current_time-prev_frame.time) / (next_frame.time-prev_frame.time);

                    let bone_location=&mut skeleton.bones_locations[anim_track.bone_index];
                    bone_location.position=prev_frame.location.position + (next_frame.location.position - prev_frame.location.position)*k;
                    bone_location.rotation=prev_frame.location.rotation + (next_frame.location.rotation - prev_frame.location.rotation)*k;
                    bone_location.scale.0=prev_frame.location.scale.0 + (next_frame.location.scale.0 - prev_frame.location.scale.0)*k;

                    *play_anim_track=Some( prev_frame_index );
                }
            }

            let last_frame=anim_track.keyframes.len() - 1;

            skeleton.bones_locations[anim_track.bone_index] = anim_track.keyframes[last_frame].location;
            *play_anim_track=None;
            self.animating_bones_count-=1;
        }

        skeleton.animated=true;

        if self.animating_bones_count==0 {
            if self.cyclic {
                self.animating_bones_count=animation_bones_tracks_guard.len();

                for play_anim_track in self.bones_tracks.iter_mut() {
                    *play_anim_track=Some(0);
                }
            }else{
                return true;
            }
        }

        false
    }
}
