use std;
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

    pub fn calculate_matrix(&self) -> Matrix4 {
        use cgmath::SquareMatrix;
        use cgmath::EuclideanSpace;

        Matrix4::from_translation(self.position.to_vec())*
        Matrix4::from(self.rotation)*
        Matrix4::from_scale(self.scale.0)
    }

}

impl PartialEq for Location {
    fn eq(&self, other:&Self) -> bool {
        const eps:f32 = 0.00001;

        let pos1=&self.position;
        let pos2=&other.position;

        if (pos1.x - pos2.x).abs() > eps || (pos1.y - pos2.y).abs() > eps || (pos1.z - pos2.z).abs() > eps {
            return false;
        }

        if (self.scale.0 - other.scale.0).abs() > eps {
            return false;
        }

        let rot1=&self.rotation;
        let rot2=&other.rotation;

        if (rot1.v.x - rot2.v.x).abs() > eps || (rot1.v.y - rot2.v.y).abs() > eps || (rot1.v.z - rot2.v.z).abs() > eps ||
            (rot1.s - rot2.s).abs() > eps
        {
            return false;
        }

        true
    }

    fn ne(&self, other:&Self) -> bool {
        !self.eq(other)
    }
}

impl std::ops::Sub for Location {
    type Output = Location;

    fn sub(self, other: Location) -> Location {
        use cgmath::EuclideanSpace;

        Location {
            position: Pos3D::from_vec(self.position-other.position),
            scale: Scale(self.scale.0/other.scale.0),
            rotation: self.rotation-other.rotation,
        }
    }
}

impl From<Matrix4> for Location {
    fn from(matrix:Matrix4) -> Self {
        /*
        let position = Position::with_asset(self.mat[3], self.mat[7], self.mat[11], asset);

        let scale_x = ((self.mat[0].powi(2) + self.mat[4].powi(2) + self.mat[8].powi(2)).sqrt()*100.0).round()/100.0;
        let scale_y = ((self.mat[1].powi(2) + self.mat[5].powi(2) + self.mat[9].powi(2)).sqrt()*100.0).round()/100.0;
        let scale_z = ((self.mat[2].powi(2) + self.mat[6].powi(2) + self.mat[10].powi(2)).sqrt()*100.0).round()/100.0;

        let scale = Scale::with_asset(scale_x, scale_y, scale_z, asset);

        let quat=self.to_quat(asset);

        Location::new(position, scale, quat)
        */

        Location::identity()//TODO:fix this
    }
}

pub fn calculate_matrix(location:&Location) -> Matrix4 {
    use cgmath::SquareMatrix;
    use cgmath::Vector3;
    use cgmath::EuclideanSpace;
    //let mut matrix=Matrix4::identity();//from(location.rotation);
    let mut matrix=Matrix4::from_translation(location.position.to_vec())*Matrix4::from(location.rotation);
    /*
    matrix[3][0]=location.position.x;
    matrix[3][1]=location.position.y;
    matrix[3][2]=location.position.z;
    */

    //Matrix4::from_scale(3.0)*matrix//*Matrix4::from_scale(3.0)
    matrix
}
