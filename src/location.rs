use cgmath;

pub type Pos2D = cgmath::Point2<f32>;
pub type Pos3D = cgmath::Point3<f32>;

#[derive(Copy,Clone)]
pub struct Scale(pub f32);

pub type Deg = cgmath::Deg<f32>;
pub type Rad = cgmath::Rad<f32>;
//pub type Euler = cgmath::Euler<f32>;
pub type Quaternion = cgmath::Quaternion<f32>;

pub type Matrix4 = cgmath::Matrix4<f32>;

#[derive(Copy,Clone)]
pub struct Location {
    pub position:Pos3D,
    pub scale:Scale,
    pub rotation:Quaternion,
}

impl Location {
    pub fn new(position:Pos3D,scale:Scale,rotation:Quaternion) -> Self {
        Location{
            position:position,
            scale:scale,
            rotation:rotation,
        }
    }

    pub fn identity() -> Self {
        Location{
            position:Pos3D::new(0.0,0.0,0.0),
            scale:Scale(1.0),
            rotation:Quaternion::new(1.0,0.0,0.0,0.0),
        }
    }
}

pub fn calculate_matrix(location:&Location) -> Matrix4 {
    use cgmath::SquareMatrix;

    //let mut matrix=Matrix4::identity();//from(location.rotation);
    let mut matrix=Matrix4::from(location.rotation);

    matrix[3][0]=location.position.x;
    matrix[3][1]=location.position.y;
    matrix[3][2]=location.position.z;

    //Matrix4::from_scale(3.0)*matrix//*Matrix4::from_scale(3.0)
    matrix
}
