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
    pub id:String,
    pub parent_index:Option<usize>,

    pub invert_matrix:Matrix4,
}

impl Skeleton {
    pub fn from_virtual<'a>(virtual_skeleton:&Option<VirtualSkeleton<'a>> ) -> Result<(Self,ZeroFrame,Vec<SkeletonVertex>,Vec<SkeletonVertex>),Error> {
        use cgmath::SquareMatrix;

        let zero_frame_locations=match *virtual_skeleton {
            Some( ref virtual_skeleton ) => {
                let mut zero_frame_locations=virtual_skeleton.zero_frame_locations.clone();

                let quat=zero_frame_locations[0].rotation;

                for loc in zero_frame_locations.iter_mut().skip(1) {
                    /*
                    let tmp=loc.position.y;
                    loc.position.y=loc.position.z;
                    loc.position.z=tmp;

                    loc.rotation=loc.rotation-quat;
                    */
                    //loc.rotation=loc.rotation+quat;
                    //loc.rotation.v.x=-loc.rotation.v.x;
                    //loc.rotation.v.y=-loc.rotation.v.y;
                    //loc.rotation.v.z=-loc.rotation.v.z;
                    /*
                    let tmp=loc.rotation.v.y;
                    loc.rotation.v.y=loc.rotation.v.z;
                    loc.rotation.v.z=tmp;
                    */
                }

                println!("AAA:{:?}", zero_frame_locations[0].rotation);

                //zero_frame_locations[0]=Location::identity();
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

                    let invert_matrix=mat.invert().unwrap();

                    //TODO:clear code and optimize

                    bones_matrices.push( mat );

                    match bones_names.insert( bone_name.clone(),bones_array.len() ) {
                        None => {},
                        Some(_) => return Err(Error::DuplicateBone( bone_name.clone() )),
                    }

                    let bone=Bone::new(
                        bone_name.clone(),
                        collada_bone.id.clone(),
                        collada_bone.parent.clone(),
                        invert_matrix,
                    );

                    bones_array.push(bone);
                }
            },
            None => {
                bones_array.push( Bone::new(
                    String::from("root"),
                    String::from("root"),
                    None,
                    Matrix4::identity().invert().unwrap()
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

        //who have children?

        let mut have_children=vec!(false;bones_array.len());
        for bone in bones_array.iter() {
            match bone.parent_index {
                Some( parent_bone_index ) => have_children[parent_bone_index] = true,
                None => {},
            };
        }

        let have_no_children_count=have_children.iter().filter(|has| !**has).count();

        //build geometry

        let mut joints_buffer=Vec::with_capacity(bones_array.len());
        let mut bones_buffer=Vec::with_capacity((bones_array.len()+have_no_children_count)*2);

        for (i,bone) in bones_array.iter().enumerate() {
            let (begin_pos,parent_index) = match bone.parent_index {
                Some( parent_bone_index ) => {
                    let vpos=bones_matrices[parent_bone_index]*Vector4::new(0.0,0.0,0.0,1.0);
                    (Pos3D::new( vpos.x, vpos.y, vpos.z ), parent_bone_index)
                },
                None =>
                    (Pos3D::new(0.0,0.0,0.0), 0)
            };

            let vpos=bones_matrices[i]*Vector4::new(0.0,0.0,0.0,1.0);
            let end_pos = Pos3D::new( vpos.x, vpos.y, vpos.z );

            joints_buffer.push(SkeletonVertex::new( end_pos, 0.7, i as u32 ));

            bones_buffer.push( SkeletonVertex::new( begin_pos, 0.7, parent_index as u32 ));
            bones_buffer.push( SkeletonVertex::new( end_pos, 0.3, i as u32 ));
        }

        for (i,bone) in bones_array.iter().enumerate() {
            if !have_children[i] {
                let vpos=bones_matrices[i]*Vector4::new(0.0,0.0,0.0,1.0);
                let begin_pos = Pos3D::new( vpos.x, vpos.y, vpos.z );

                let vpos=bones_matrices[i]*Vector4::new(0.0,0.0,0.3,1.0);
                let end_pos = Pos3D::new( vpos.x, vpos.y, vpos.z );

                bones_buffer.push(SkeletonVertex::new( begin_pos, 0.7, i as u32 ));
                bones_buffer.push(SkeletonVertex::new( end_pos, 0.1, i as u32 ));
            }
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
        id:String,
        parent_index:Option<usize>,
        invert_matrix:Matrix4,
    ) -> Self {
        Bone{
            name:name,
            id:id,
            parent_index:parent_index,
            invert_matrix:invert_matrix,
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
