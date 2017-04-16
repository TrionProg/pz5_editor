use render;
use location::*;

use object_pool::growable::{ID,Slot};
use std::collections::HashMap;
use from_collada::VirtualSkeleton;

use super::Error;

use super::ZeroFrame;

use render::skeleton::Vertex as SkeletonVertex;

pub struct Skeleton {
    pub bones:Vec<Bone>,
    pub matrices:Vec<Matrix4>,
    pub skeleton_id:Option<ID>,
    pub animated:bool, //true - we need recalculate matrices
}

impl Skeleton {
    pub fn from_virtual<'a>(virtual_skeleton:&Option<VirtualSkeleton<'a>>, model_location:&Location ) -> Result<(Self,ZeroFrame,Vec<SkeletonVertex>,Vec<SkeletonVertex>),Error> {
        use cgmath::SquareMatrix;

        let (bones_count, mut bones_array, bones_names, mut bones_matrices) = Self::load_bones_from_virtual(virtual_skeleton, model_location)?;
        let (joints_buffer, bones_buffer) = Self::build_geometry(&bones_array, &mut bones_matrices);


        let zero_frame=match *virtual_skeleton {
            Some( ref virtual_skeleton ) => ZeroFrame::new( virtual_skeleton.zero_frame_locations.clone() ),
            None =>
                ZeroFrame::new( Vec::new() ),
        };

        let skeleton=Skeleton{
            bones:bones_array,
            matrices:bones_matrices,
            skeleton_id:None,
            animated:true,
        };

        Ok( (skeleton, zero_frame, joints_buffer, bones_buffer) )
    }

    fn load_bones_from_virtual<'a>(virtual_skeleton:&Option<VirtualSkeleton<'a>>, model_location:&Location) -> Result<(usize, Vec<Bone>, HashMap<String,usize>, Vec<Matrix4>),Error> {
        use cgmath::SquareMatrix;

        let bones_count=match *virtual_skeleton {
            Some( ref virtual_skeleton ) => virtual_skeleton.collada_skeleton.bones_array.len()+1,
            None => 1,
        };

        let mut bones_array=Vec::with_capacity(bones_count);
        let mut bones_names=HashMap::new();
        let mut bones_matrices=Vec::with_capacity(bones_count);

        bones_array.push( Bone::new(
            String::from("root"),
            None,
            model_location.clone(),//but matrix is identity
        ));
        bones_names.insert(String::from("root"),0);
        bones_matrices.push(Matrix4::identity());

        match *virtual_skeleton {
            Some( ref virtual_skeleton ) => {
                for (location,collada_bone) in virtual_skeleton.zero_frame_locations.iter().zip( virtual_skeleton.collada_skeleton.bones_array.iter() ) {
                    let bone_name=&collada_bone.name;

                    let parent=match collada_bone.parent {
                        Some( bone_index ) => bone_index+1,
                        None => 0,
                    };

                    //use cgmath::SquareMatrix;
                    use cgmath::Vector3;
                    use cgmath::EuclideanSpace;

                    let m = Matrix4::from(location.rotation)*
                            Matrix4::from_translation(location.position.to_vec())*
                            Matrix4::from_scale(location.scale.0);

                    let mat=bones_matrices[parent]*m;
                    bones_matrices.push( mat );

                    match bones_names.insert( bone_name.clone(),bones_array.len() ) {
                        None => {},
                        Some(_) => unreachable!()//return Err(Error::DuplicateBone( bone_name.clone() )),
                    }

                    let bone=Bone::new(
                        bone_name.clone(),
                        Some(parent),

                        location.clone()
                    );

                    bones_array.push(bone);
                }
            },
            None => {},
        };

        Ok( (bones_count, bones_array, bones_names, bones_matrices) )
    }

    fn build_geometry(bones_array:&Vec<Bone>, bones_matrices:&mut Vec<Matrix4>) -> (Vec<render::skeleton::Vertex>,Vec<render::skeleton::Vertex>) {
        use cgmath::Vector3;
        use cgmath::Vector4;
        use cgmath::SquareMatrix;
        use cgmath::EuclideanSpace;
        use cgmath::BaseNum;

        //Calculate matrices
        bones_matrices[0]=Matrix4::identity();

        for (i,bone) in bones_array.iter().skip(1).enumerate() {
            let m = Matrix4::from(bone.location.rotation)*
                    Matrix4::from_translation(bone.location.position.to_vec())*
                    Matrix4::from_scale(bone.location.scale.0);

            bones_matrices[i]=bones_matrices[ bone.parent_index.unwrap() ] * m;
        }

        let mut joints_buffer=Vec::with_capacity(bones_array.len());
        let mut bones_buffer=Vec::with_capacity(bones_array.len()*2);

        for (i,bone) in bones_array.iter().enumerate() {
            //let parent
            let begin_pos = Pos3D::from_homogeneous( bones_matrices[i]*Vector4::new(0.0,0.0,0.0,1.0) );
            let end_pos = Pos3D::from_homogeneous(bones_matrices[i]*bone.location.position.to_homogeneous());

            joints_buffer.push(SkeletonVertex::new(
                begin_pos,
                0.7,
                i as u32
            ));

            bones_buffer.push(SkeletonVertex::new(
                begin_pos,
                0.7,
                i as u32
            ));
            bones_buffer.push(SkeletonVertex::new(
                end_pos,
                0.3,
                i as u32
            ));
        }

        for i in joints_buffer.iter(){
            println!("{} {}",i.bone_index, i.color);
        }

        //bones_buffer.truncate(12);
        //joints_buffer[11].bone_index=3;
        //joints_buffer[11].color=0.0;

        (joints_buffer, bones_buffer)
    }

    pub fn calculate_matrices(&mut self, frame:&mut render::Frame) -> Result<(),render::Error> {
        if !self.animated {
            return Ok(());
        }

        self.animated=false;

        //self.bones[1].location.scale.0=1.5;

        //self.bones[11].location.position.z=2.0;
        //self.bones[11].location.position.y=-0.5;

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


    pub fn render(&self, frame:&mut render::Frame) -> Result<(),render::Error> {
        //println!("{}",self.bones.len());
        let skeleton_id = match self.skeleton_id {
            Some( ref skeleton_id ) => *skeleton_id,
            None => return Err(render::Error::NoSkeleton),
        };

        let skeleton=match frame.storage.skeletons.get(skeleton_id) {
            Some( skeleton ) => skeleton,
            None => return Err(render::Error::NoGeometryWithID(skeleton_id)),
        };
        //let skeleton=self.get_skeleton_from_storage(frame)?;

        skeleton.render( frame )
    }
}

pub struct Bone {
    pub name:String,
    pub parent_index:Option<usize>,

    pub location:Location,

    pub matrix:Matrix4,
}

impl Bone {
    pub fn new(
        name:String,
        parent_index:Option<usize>,
        location:Location,
    ) -> Self {
        use cgmath::SquareMatrix;

        Bone{
            name:name,
            parent_index:parent_index,

            location:location,

            matrix:Matrix4::identity(),
        }
    }

    pub fn calculate_matrix(&self, matrices:&Vec<Matrix4>) -> Matrix4 {
        let bone_matrix=calculate_matrix(&self.location);

        let final_bone_matrix=match self.parent_index {
            Some( parent_index ) => matrices[parent_index] * bone_matrix,
            _ => bone_matrix,
        };

        final_bone_matrix
    }
/*
    pub fn build_geometry(&self, i:usize, joints_buffer:&mut Vec<render::skeleton::Vertex>, bones_buffer:&mut Vec<render::skeleton::Vertex>){
        let parent_index=match self.parent_index {
            Some( parent_index ) => parent_index,
            None => 0,
        };

        joints_buffer.push( render::skeleton::Vertex::new(i as u32,0.3) );

        bones_buffer.push( render::skeleton::Vertex::new(parent_index as u32,0.3) );
        bones_buffer.push( render::skeleton::Vertex::new(i as u32,0.7) );
    }
*/
}
