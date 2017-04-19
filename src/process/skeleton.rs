use render;
use location::*;

use object_pool::growable::{ID,Slot};
use std::collections::HashMap;
use from_collada::VirtualSkeleton;

use super::Error;

use super::ZeroFrame;
use super::SkeletonOfInstance;

use render::skeleton::Vertex as SkeletonVertex;

pub struct Skeleton {
    pub bones_array:Vec<Bone>,
    pub bones_names:HashMap<String,usize>,
    pub geometry_of_skeleton_id:Option<ID>,
}

pub struct Bone {
    pub name:String,
    pub parent_index:Option<usize>,

    pub matrix:Matrix4,
}

impl Skeleton {
    pub fn from_virtual<'a>(virtual_skeleton:&Option<VirtualSkeleton<'a>> ) -> Result<(Self,ZeroFrame,Vec<SkeletonVertex>,Vec<SkeletonVertex>),Error> {
        use cgmath::SquareMatrix;

        let zero_frame_locations=match *virtual_skeleton {
            Some( ref virtual_skeleton ) => {
                let mut zero_frame_locations=virtual_skeleton.zero_frame_locations.clone();
                zero_frame_locations[0]=Location::identity();
                zero_frame_locations
            },
            None => {
                let mut zero_frame_locations=Vec::with_capacity(1);
                zero_frame_locations.push(Location::identity());
                zero_frame_locations
            },
        };
        let mut bones_matrices=Vec::with_capacity(zero_frame_locations.len());

        let (bones_array, bones_names) = Self::load_bones_from_virtual(virtual_skeleton, &zero_frame_locations, &mut bones_matrices)?;
        let (joints_buffer, bones_buffer) = Self::build_geometry(&bones_array, &zero_frame_locations, &mut bones_matrices);


        let zero_frame=ZeroFrame::new( zero_frame_locations );

        let skeleton=Skeleton{
            bones_array:bones_array,
            bones_names:bones_names,
            geometry_of_skeleton_id:None,
        };

        Ok( (skeleton, zero_frame, joints_buffer, bones_buffer) )
    }

    fn load_bones_from_virtual<'a>(virtual_skeleton:&Option<VirtualSkeleton<'a>>, zero_frame_locations:&Vec<Location>, bones_matrices:&mut Vec<Matrix4>) -> Result<(Vec<Bone>, HashMap<String,usize>),Error> {
        use cgmath::SquareMatrix;

        let bones_count=zero_frame_locations.len();

        let mut bones_array=Vec::with_capacity(bones_count);
        let mut bones_names=HashMap::new();
        let mut bones_matrices=Vec::with_capacity(bones_count);

        match *virtual_skeleton {
            Some( ref virtual_skeleton ) => {
                let mut is_none_parent=false;
                for collada_bone in virtual_skeleton.collada_skeleton.bones_array.iter() {
                    if collada_bone.parent.is_none() {
                        if is_none_parent {
                            return Err(Error::SkeletonHasSeparatedBranches( String::from("aa")/*virtual_skeleton.get_name().clone()*/ ));//TODO:fix this
                        }

                        is_none_parent=true;
                    }
                }

                for (location,collada_bone) in zero_frame_locations.iter().zip( virtual_skeleton.collada_skeleton.bones_array.iter() ) {
                    let bone_name=&collada_bone.name;
                    //use cgmath::SquareMatrix;
                    use cgmath::Vector3;
                    use cgmath::EuclideanSpace;

                    let m = Matrix4::from_translation(location.position.to_vec())*
                            Matrix4::from(location.rotation)*
                            Matrix4::from_scale(location.scale.0);

                    let mat = match collada_bone.parent {
                        Some( parent_bone_index ) => bones_matrices[parent_bone_index] * m,
                        None => m,
                    };

                    //TODO:calculate inverse matrix

                    bones_matrices.push( mat );

                    match bones_names.insert( bone_name.clone(),bones_array.len() ) {
                        None => {},
                        Some(_) => return Err(Error::DuplicateBone( bone_name.clone() )),
                    }

                    let bone=Bone::new(
                        bone_name.clone(),
                        collada_bone.parent.clone()
                    );

                    bones_array.push(bone);
                }
            },
            None => {
                bones_array.push( Bone::new(
                    String::from("root"),
                    None
                ));
                bones_names.insert(String::from("root"),0);
                bones_matrices.push(Matrix4::identity());
            },
        };

        Ok( (bones_array, bones_names) )
    }

    fn build_geometry(bones_array:&Vec<Bone>, zero_frame_locations:&Vec<Location>, bones_matrices:&mut Vec<Matrix4>) -> (Vec<render::skeleton::Vertex>,Vec<render::skeleton::Vertex>) {
        use cgmath::Vector3;
        use cgmath::Vector4;
        use cgmath::SquareMatrix;
        use cgmath::EuclideanSpace;
        use cgmath::BaseNum;

        //Calculate matrices
        bones_matrices.clear();

        for (location,bone) in zero_frame_locations.iter().zip( bones_array.iter() ) {
            let m = Matrix4::from_translation(location.position.to_vec())*
                    Matrix4::from(location.rotation)*
                    Matrix4::from_scale(location.scale.0);

            let mat = match bone.parent_index {
                Some( parent_bone_index ) => bones_matrices[parent_bone_index] * m,
                None => m,
            };

            bones_matrices.push( mat );
        }

        let mut joints_buffer=Vec::with_capacity(bones_array.len());
        let mut bones_buffer=Vec::with_capacity(bones_array.len()*2);

        for (i,(location,bone)) in zero_frame_locations.iter().zip( bones_array.iter() ).enumerate() {
            let begin_pos = match bone.parent_index {
                Some( parent_bone_index ) =>
                    Pos3D::from_homogeneous( bones_matrices[parent_bone_index]*Vector4::new(0.0,0.0,0.0,1.0) ),
                None =>
                    Pos3D::new(0.0,0.0,0.0),
            };

            let end_pos = Pos3D::from_homogeneous(bones_matrices[i]*location.position.to_homogeneous());

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
            println!("{} {} | {} {} {}",i.bone_index, i.color, i.position[0],i.position[1],i.position[2]);
        }

        (joints_buffer, bones_buffer)
    }

    pub fn get_geometry_of_skeleton_from_storage<'a>(&self, frame:&render::Frame<'a>) -> Result<&'a render::GeometryOfSkeleton,render::Error> {
        let geometry_of_skeleton_id = match self.geometry_of_skeleton_id {
            Some( ref geometry_of_skeleton_id ) => *geometry_of_skeleton_id,
            None => return Err(render::Error::NoSkeleton),
        };

        frame.storage.get_geometry_of_skeleton(geometry_of_skeleton_id)
    }

    pub fn render(&self, frame:&mut render::Frame, skeleton_of_instance:&SkeletonOfInstance) -> Result<(),render::Error> {
        let ren_skeleton_of_instance=skeleton_of_instance.get_skeleton_of_instance_from_storage(frame)?;
        let ren_geometry_of_skeleton=self.get_geometry_of_skeleton_from_storage(frame)?;
        /*
        //println!("{}",self.bones.len());
        let skeleton_geometry_id = match self.skeleton_geometry_id {
            Some( ref skeleton_geometry_id ) => *skeleton_geometry_id,
            None => return Err(render::Error::NoSkeleton),
        };

        let instance_skeleton_id = match instance_skeleton.skeleton_id {
            Some( ref skeleton_geometry_id ) => *skeleton_geometry_id,
            None => return Err(render::Error::NoSkeleton),
        };

        let skeleton_geometry=match frame.storage.skeletons_geometry.get(skeleton_geometry) {
            Some( skeleton_geometry ) => skeleton_geometry,
            None => return Err(render::Error::NoGeometryWithID(skeleton_id)),
        };

        let skeleton=match frame.storage.skeletons.get(skeleton) {
            Some( skeleton_geometry ) => skeleton_geometry,
            None => return Err(render::Error::NoGeometryWithID(skeleton_id)),
        };
        */
        //let skeleton=self.get_skeleton_from_storage(frame)?;

        ren_geometry_of_skeleton.render( frame,ren_skeleton_of_instance )
    }
}

impl Bone {
    pub fn new(
        name:String,
        parent_index:Option<usize>,
    ) -> Self {
        use cgmath::SquareMatrix;

        Bone{
            name:name,
            parent_index:parent_index,

            matrix:Matrix4::identity(),
        }
    }
/*
    pub fn calculate_matrix(&self, matrices:&Vec<Matrix4>) -> Matrix4 {
        let bone_matrix=calculate_matrix(&self.location);

        let final_bone_matrix=match self.parent_index {
            Some( parent_index ) => matrices[parent_index] * bone_matrix,
            _ => bone_matrix,
        };

        final_bone_matrix
    }
*/
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
