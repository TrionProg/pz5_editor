use std;
use collada;
use location::*;

use std::sync::Arc;

use super::Error;

pub struct VirtualSkeleton<'a> {
    pub collada_skeleton:&'a Arc<collada::Skeleton>,
    pub zero_frame_locations:Vec<Location>,
}

impl<'a> VirtualSkeleton<'a> {
    pub fn new(model_location:&Location, collada_skeleton:&'a Arc<collada::Skeleton>) -> Result<Self,Error> {
        let mut locations=Vec::with_capacity(collada_skeleton.bones_array.len());

        for bone in collada_skeleton.bones_array.iter() {
            let bone_location = match Location::from_collada(&bone.location) {
                Ok ( location ) => location,
                Err( _ ) => {
                    return Err( Error::Other( format!("Bone \"{}\" of skeleton \"{}\" has different scales",bone.name,collada_skeleton.id) ));
                },
            };

            locations.push( bone_location );
        }

        let virtual_skeleton=VirtualSkeleton{
            collada_skeleton:collada_skeleton,
            zero_frame_locations:locations,
        };

        Ok( virtual_skeleton )
    }
}

impl<'a> PartialEq for VirtualSkeleton<'a> {
    fn eq(&self, other:&Self) -> bool {
        if self.collada_skeleton.bones_array.len() != other.collada_skeleton.bones_array.len() {
            return false;
        }

        for (((bone1,bone2),location1),location2) in self.collada_skeleton.bones_array.iter()
            .zip(other.collada_skeleton.bones_array.iter())
            .zip(self.zero_frame_locations.iter())
            .zip(other.zero_frame_locations.iter()) {
            if bone1.id != bone2.id || bone1.name != bone2.name || location1 != location2 || bone1.parent != bone2.parent {
                return false;
            }
        }

        true
    }

    fn ne(&self, other:&Self) -> bool {
        !self.eq(other)
    }
}
