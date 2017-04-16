use location::*;

pub struct ZeroFrame{
    bones:Vec<Location>,
}

impl ZeroFrame {
    pub fn new( bones:Vec<Location> ) -> ZeroFrame {
        ZeroFrame {
            bones:bones,
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
    bones:Vec<(usize,Vec<KeyFrame>)>,
}
