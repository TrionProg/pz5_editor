use location::*;

pub struct Skeleton {
    bones:Vec<Bone>,
    animated:bool, //true - we need recalculate matrices
}

impl Skeleton {
    pub fn new( bones:Vec<Bone> ) -> Self {
        Skeleton{
            bones:bones,
            animated:true,
        }
    }

    pub fn calculate_matrices(&mut self) {
        if !self.animated {
            return;
        }

        for i in 0..self.bones.len() {
            /*
            let bone_matrix=calculate_by_pos_scale_quat(&self.bones[i].position, &self.bones[i].scale, &self.bones[i].rotation);

            let final_bone_matrix=match self.bones[i].parent_index {
                Some( parent_index ) => self.bones[parent_index].matrix * bone_matrix,
                _ => bone_matrix,
            };

            self.bones[i].matrix=final_bone_matrix,
            */
        }
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
}
