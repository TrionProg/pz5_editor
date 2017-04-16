/*
use std;
use pz5;
use collada;
use location::*;

use std::collections::HashSet;
use std::sync::Arc;

use collada::Document;
use collada::Scene;

use object::ZeroFrame;
use object::Skeleton;
use object::Bone;

use super::Error;

pub fn read_skeleton(scene:&Scene) -> Result<(Skeleton,ZeroFrame), Error> {
    let mut bones=Vec::new();
    let mut bones_names=HashSet::new();
    let mut zero_frame_bones=Vec::new();

    let location = Location::identity();

    let bone=Bone::new(
        String::from("model"),
        None,

        location,
    );

    bones.push( bone );
    zero_frame_bones.push( location );

    let multiple_skeletons=scene.skeletons.len()>1;

    for (_,skeleton_node) in scene.skeletons.iter() {
        let collada_skeleton=&skeleton_node.joined;

        let location = match Location::from_collada(&skeleton_node.location) {
            Ok( loc ) => loc,
            Err( _ ) => return Err( Error::SkeletonDifferentSizes( skeleton_node.name.clone() )),
        };

        let bone=Bone::new(
            skeleton_node.name.clone(),
            Some(0),

            location,
        );

        let skeleton_bone_index=bones.len();
        bones.push( bone );
        zero_frame_bones.push( location );

        let mut index_offset=bones.len();

        for collada_bone in collada_skeleton.bones_array.iter() {
            let bone_name=if multiple_skeletons {
                format!("{}_{}", skeleton_node.name, collada_bone.name)
            }else{
                collada_bone.name.clone()
            };

            let parent=match collada_bone.parent {
                Some( bone_index ) => index_offset + bone_index,
                None => skeleton_bone_index,
            };

            let location = match Location::from_collada(&collada_bone.location) {
                Ok( loc ) => loc,
                Err( _ ) => return Err( Error::BoneDifferentSizes( collada_bone.name.clone(), skeleton_node.name.clone() )),
            };

            match bones_names.insert( bone_name.clone() ) {
                true => {},
                false => return Err( Error::DuplicateBone(bone_name) ),
            }

            let bone=Bone::new(
                bone_name,
                Some(parent),

                location
            );

            bones.push( bone );
            zero_frame_bones.push( location );
        }
    }

    Ok(( Skeleton::new( bones ), ZeroFrame::new(zero_frame_bones) ))
}
*/
