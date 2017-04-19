use location::*;

pub struct ZeroFrame{
    pub bones_locations:Vec<Location>,
}

impl ZeroFrame {
    pub fn new( bones_locations:Vec<Location> ) -> ZeroFrame {
        ZeroFrame {
            bones_locations:bones_locations,
        }
    }
}

pub struct KeyFrame{
    pub time:f32,
    pub position:Pos3D,
    pub scale:Scale,
    pub rotation:Quaternion,
}

pub struct Animation{
    pub bones:Vec<(usize,Vec<KeyFrame>)>,
}
