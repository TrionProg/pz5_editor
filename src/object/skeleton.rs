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

        for i in 0..self.bones.len() {
            let bone_matrix=self.bones[i].calculate_matrix(&self.matrices);
            use cgmath::SquareMatrix;
            self.matrices[i]=bone_matrix;
        }

        let skeleton=self.get_skeleton_from_storage(frame)?;

        skeleton.load_bones( &self.matrices[..] );

        Ok(())
    }

    pub fn build_geometry(&self) -> Vec<storage::skeleton::Vertex> {
        let mut buffer=Vec::with_capacity(self.bones.len()*2);

        for i in 0..self.bones.len() {
            self.bones[i].build_geometry(i,&mut buffer);
        }

        buffer
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

    pub fn build_geometry(&self, i:usize, buffer:&mut Vec<storage::skeleton::Vertex>){
        let parent_index=match self.parent_index {
            Some( parent_index ) => parent_index,
            None => 0,
        };

        buffer.push( storage::skeleton::Vertex::new(parent_index as u32,0.3) );
        buffer.push( storage::skeleton::Vertex::new(i as u32,0.7) );
    }
}
