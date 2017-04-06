use collada;
use cgmath;

use location::*;

use super::Error;

impl Location {
    pub fn from_collada(collada_location:&collada::Location) -> Result<Self,()> {
        let scale=Scale::from_collada(&collada_location.scale)?;

        let location=Location{
            position:pos3d_from_collada(&collada_location.position),
            scale:scale,
            rotation:quaternion_from_collada(&collada_location.rotation),
        };

        Ok( location )
    }
}

pub fn pos3d_from_collada(collada_position:&collada::Position) -> Pos3D {
    Pos3D::new(
        collada_position.x,
        collada_position.y,
        collada_position.z
    )
}

pub fn quaternion_from_collada(collada_quat:&collada::Quaternion) -> Quaternion {
    Quaternion::new(
        collada_quat.w,
        collada_quat.x,
        collada_quat.y,
        collada_quat.z
    )
}

impl Scale {
    pub fn from_collada(collada_scale:&collada::Scale) -> Result<Self, ()> {
        let scale_x = collada_scale.x;
        let scale_y = collada_scale.y;
        let scale_z = collada_scale.z;

        if scale_x != scale_y || scale_y != scale_z || scale_z != scale_x {
            return Err( () );
        }

        Ok( Scale(scale_x) )
    }
}



/*
pub fn pos_scale_quat_from_collada_matrix(matrix:&Matrix) -> Result<(Pos3D,Scale,Quaternion),()> {
    let position = Pos3D::new(
        matrix.mat[3],
        matrix.mat[7],
        matrix.mat[11],
    );

    println!("{} {} {}", position.x, position.y, position.z);

    let scale_x = ((matrix.mat[0].powi(2) + matrix.mat[4].powi(2) + matrix.mat[8].powi(2)).sqrt()*100.0).round()/100.0;
    let scale_y = ((matrix.mat[1].powi(2) + matrix.mat[5].powi(2) + matrix.mat[9].powi(2)).sqrt()*100.0).round()/100.0;
    let scale_z = ((matrix.mat[2].powi(2) + matrix.mat[6].powi(2) + matrix.mat[10].powi(2)).sqrt()*100.0).round()/100.0;

    if scale_x != scale_y || scale_y != scale_z || scale_z != scale_x {
        return Err( () );
    }

    let scale:Scale = scale_x;

    let rot_matrix=cgmath::Matrix3::new(
        matrix.mat[0]/scale_x, matrix.mat[4]/scale_x, matrix.mat[8]/scale_x,
        matrix.mat[1]/scale_y, matrix.mat[5]/scale_y, matrix.mat[9]/scale_y,
        matrix.mat[2]/scale_z, matrix.mat[6]/scale_z, matrix.mat[10]/scale_z
    );

    let quat=Quaternion::from( rot_matrix );

    println!("{} {} {} {}", quat.v.x, quat.v.y, quat.v.z, quat.s);

    Ok( (position, scale, quat) )
}
*/
