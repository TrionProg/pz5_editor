use std;
use collada;
use process;
use location::*;

use super::Error;

pub fn read_bone_track(collada_animation:&collada::Animation) -> Result<Vec<process::KeyFrame>,Error> {
    let mut keyframes=Vec::with_capacity(collada_animation.keyframes_count);

    let interpolations=match collada_animation.sources.get("INTERPOLATION") {
        Some( interpolations_source ) => {
            match interpolations_source.layers.get("INTERPOLATION") {
                Some( interpolations_layer ) => {
                    match *interpolations_layer {
                        collada::SourceLayer::Name( ref interpolations ) => interpolations,
                        _ => return Err(Error::LayerMustBeName( "INTERPOLATION".to_string() )),
                    }
                },
                None =>
                    return Err(Error::NoLayer( "INTERPOLATION".to_string(), "INTERPOLATION".to_string() )),
            }
        },
        None =>
            return Err(Error::NoSource( "INTERPOLATION".to_string() )),
    };

    let times=match collada_animation.sources.get("INPUT") {
        Some( times_source ) => {
            match times_source.layers.get("TIME") {
                Some( times_layer ) => {
                    match *times_layer {
                        collada::SourceLayer::F32( ref times ) => times,
                        _ => return Err(Error::LayerMustBeF32( "TIME".to_string() )),
                    }
                },
                None =>
                    return Err(Error::NoLayer( "TIME".to_string(), "INPUT".to_string() )),
            }
        },
        None =>
            return Err(Error::NoSource( "INPUT".to_string() )),
    };

    let locations=match collada_animation.sources.get("OUTPUT") {
        Some( times_source ) => {
            match times_source.layers.get("location") {
                Some( times_layer ) => {
                    match *times_layer {
                        collada::SourceLayer::Location( ref times ) => times,
                        _ => return Err(Error::LayerMustBeLocation( "TRANSFORM".to_string() )),
                    }
                },
                None =>
                    return Err(Error::NoLayer( "TRANSFORM".to_string(), "OUTPUT".to_string() )),
            }
        },
        None =>
            return Err(Error::NoSource( "OUTPUT".to_string() )),
    };

    for (time,(location,interpolation)) in times.iter().zip( locations.iter().zip( interpolations.iter() ) ) {
        if interpolation.as_str() != "LINEAR" {
            return Err(Error::UnsupportedInterpolation( interpolation.clone() ));
        }

        let location = match Location::from_collada(location) {
            Ok ( location ) => location,
            Err( _ ) => {
                return Err( Error::Other( format!("Location of bone \"{}\" with animation time \"{}\" has different scales", collada_animation.bone_id, *time) ));
            },
        };

        keyframes.push( process::KeyFrame::new(*time,location) );
    }

    if keyframes.len() == 0 {
        return Err(Error::AnimationWithoutKeyframes( collada_animation.bone_id.clone() ));
    }

    keyframes.sort_by(|keyframe1,keyframe2| keyframe1.time.partial_cmp(&keyframe2.time).unwrap());

    Ok( keyframes )
}
