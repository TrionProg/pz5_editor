use std;
use location::*;
use from_collada;

use std::sync::Arc;
use std::sync::RwLock;
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::time::Instant;

use super::Error;
use super::Skeleton;

use from_collada::read_bone_track;

pub struct ZeroFrame{
    pub bones_locations:Vec<Location>,
}

pub struct KeyFrame{
    pub time:f32,
    pub location:Location,
}

pub struct Track {
    pub bone_index:usize,
    pub keyframes:Vec<KeyFrame>,
}

pub struct Animation{
    pub name:String,
    pub bones_tracks:RwLock< Vec<Track> >,
}

impl ZeroFrame {
    pub fn new( bones_locations:Vec<Location> ) -> ZeroFrame {
        ZeroFrame {
            bones_locations:bones_locations,
        }
    }
}

impl KeyFrame {
    pub fn new(time:f32, location:Location) -> Self {
        KeyFrame {
            time:time,
            location:location,
        }
    }
}

impl Track {
    pub fn new(bone_index:usize, keyframes:Vec<KeyFrame>) -> Self {
        Track {
            bone_index:bone_index,
            keyframes:keyframes,
        }
    }
}

impl Animation {
    pub fn new(name:String, bones_tracks:Vec<Track>) -> Self{
        Animation{
            name:name,
            bones_tracks:RwLock::new( bones_tracks ),
        }
    }

    pub fn from_virtual_animations(virtual_animations:&Vec< from_collada::VirtualAnimation >, skeleton:&Skeleton) -> Result<HashMap<String,Arc<Animation>>, Error> {
        let mut animations=HashMap::new();

        for virtual_animation in virtual_animations.iter() {
            let mut bones_tracks=Vec::with_capacity( virtual_animation.bones_tracks.len() );

            for (bone_index,bone) in skeleton.bones_array.iter().enumerate() {
                for collada_animation in virtual_animation.bones_tracks.iter() {
                    if collada_animation.bone_id == bone.id {
                        let keyframes=read_bone_track( collada_animation )?;
                        bones_tracks.push( Track::new(bone_index,keyframes) );

                        break;
                    }
                }
            }

            if bones_tracks.len()==0 {
                return Err(Error::FromColladaError(
                    Box::new(from_collada::Error::AnimationWithoutTracks( virtual_animation.name.clone() ))
                ));
            }

            bones_tracks.sort_by(|track1,track2| track1.bone_index.partial_cmp(&track2.bone_index).unwrap());

            if bones_tracks[0].bone_index==0 {
                let root_track=&mut bones_tracks[0];
                for root_keyframe in root_track.keyframes.iter_mut() {
                    root_keyframe.location.position=Pos3D::new(3.0, 1.0, -1.5);
                    root_keyframe.location.rotation=Quaternion::new(1.0,0.0,0.0,0.0);
                }
            }

            //TODO:multiple bones_tracks root to virtual_animation.location

            let animation=Animation::new(virtual_animation.name.clone(), bones_tracks);

            match animations.entry(animation.name.clone()) {
                Entry::Vacant(e) => {e.insert(Arc::new( animation ));},
                Entry::Occupied(_) => return Err(Error::DuplicateAnimation( animation.name.clone() )),
            }
        }

        Ok( animations )
    }
}
