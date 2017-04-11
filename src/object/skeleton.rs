use storage;
use location::*;

use object_pool::growable::{ID,Slot};

use RenderFrame;
use RenderError;

pub struct Skeleton {
    pub bones:Vec<Bone>,
    pub matrices:Vec<Matrix4>,
    pub skeleton_id:Option<ID>,
    pub animated:bool, //true - we need recalculate matrices
}

impl Skeleton {
    pub fn new( bones:Vec<Bone> ) -> Self {
        use cgmath::SquareMatrix;
        let bones_count=bones.len();

        Skeleton{
            bones:bones,
            matrices:vec!(Matrix4::identity();bones_count),//todo what to do with limit of matrices. tank has a lot of bones, but others not so muchZz
            skeleton_id:None,
            animated:true,
        }
    }

    pub fn calculate_matrices(&mut self, frame:&mut RenderFrame) -> Result<(),RenderError> {
        if !self.animated {
            return Ok(());
        }

        self.animated=false;

        //self.bones[1].location.scale.0=1.5;

        //self.bones[11].location.position.z=2.0;
        //self.bones[11].location.position.y=-0.5;

        for i in 0..self.bones.len() {
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

            self.matrices[i]=m;//*Matrix4::from(self.bones[i].location.rotation);
            /*
            let bone_matrix=self.bones[i].calculate_matrix(&self.matrices);
            use cgmath::SquareMatrix;
            self.matrices[i]=bone_matrix;
            */

            println!("{} {:?}",i,self.bones[i].parent_index);
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

    pub fn build_geometry(&self) -> (Vec<storage::skeleton::Vertex>,Vec<storage::skeleton::Vertex>) {
        let mut joints_buffer=Vec::with_capacity(self.bones.len());
        let mut bones_buffer=Vec::with_capacity(self.bones.len()*2);

        for (i,bone) in self.bones.iter().enumerate() {
            bone.build_geometry(i,&mut joints_buffer,&mut bones_buffer);
        }

        for i in joints_buffer.iter(){
            println!("{} {}",i.bone_index, i.color);
        }

        //bones_buffer.truncate(12);
        //joints_buffer[11].bone_index=3;
        //joints_buffer[11].color=0.0;

        (joints_buffer, bones_buffer)
    }

    fn get_skeleton_from_storage<'a>(&self, frame:&'a RenderFrame ) -> Result<&'a storage::Skeleton,RenderError> {
        let skeleton_id = match self.skeleton_id {
            Some( ref skeleton_id ) => *skeleton_id,
            None => return Err(RenderError::NoSkeleton),
        };

        let skeleton=match frame.storage.skeletons.get(skeleton_id) {
            Some( skeleton ) => skeleton,
            None => return Err(RenderError::NoGeometryWithID(skeleton_id)),
        };

        Ok(skeleton)
    }


    pub fn render(&self, frame:&mut RenderFrame) -> Result<(),RenderError> {
        //println!("{}",self.bones.len());
        let skeleton_id = match self.skeleton_id {
            Some( ref skeleton_id ) => *skeleton_id,
            None => return Err(RenderError::NoSkeleton),
        };

        let skeleton=match frame.storage.skeletons.get(skeleton_id) {
            Some( skeleton ) => skeleton,
            None => return Err(RenderError::NoGeometryWithID(skeleton_id)),
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

    pub fn build_geometry(&self, i:usize, joints_buffer:&mut Vec<storage::skeleton::Vertex>, bones_buffer:&mut Vec<storage::skeleton::Vertex>){
        let parent_index=match self.parent_index {
            Some( parent_index ) => parent_index,
            None => 0,
        };

        joints_buffer.push( storage::skeleton::Vertex::new(i as u32,0.3) );

        bones_buffer.push( storage::skeleton::Vertex::new(parent_index as u32,0.3) );
        bones_buffer.push( storage::skeleton::Vertex::new(i as u32,0.7) );
    }
}
