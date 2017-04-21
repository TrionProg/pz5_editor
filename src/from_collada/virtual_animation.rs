use std;
use collada;

use std::sync::Arc;
use location::Location;

pub struct VirtualAnimation<'a> {
    pub name:String,
    pub location:Location,
    pub bones_tracks:Vec<&'a Arc<collada::Animation>>,
}

impl<'a> VirtualAnimation<'a> {
    pub fn new(location:&Location) -> Self {
        VirtualAnimation {
            name:String::new(),
            location:location.clone(),
            bones_tracks:Vec::new(),
        }
    }
}
