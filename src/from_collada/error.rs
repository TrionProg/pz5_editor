use std;
use pz5;
use collada;

#[derive(Debug)]
pub enum Error{
    ColladaError(collada::Error),
    StringParseError(String),
    SemanticsParse(String),
    ByteOrderError(std::io::Error),
    NoVertices,
    NoPolygons,
    LayerMustBeI32OrF32(&'static str),
    BoneDifferentSizes(String,String),
    SkeletonDifferentSizes(String),
    MeshDifferentSizes(String),
    DuplicateBone(String),
    Other(String),
}

impl std::fmt::Display for Error{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self{
            Error::ColladaError(ref e) => write!(f, "Collada error:{}", e),
            Error::StringParseError(ref message) => write!(f, "String parse error: {}", message),
            Error::SemanticsParse(ref e) => write!(f, "Semantics parse error:{}", e),
            Error::ByteOrderError(ref e) => write!(f, "Byte order error:{}", e),
            Error::NoVertices => write!(f, "No vertices"),
            Error::NoPolygons => write!(f, "No polygons"),
            Error::LayerMustBeI32OrF32(found) => write!(f, "Expected only f32 or i32, but layer has {} format",found),
            Error::BoneDifferentSizes(ref skeleton_name, ref bone_name) => write!(f, "Bone \"{}\" of skeleton \"{}\", has different sizes", bone_name, skeleton_name),
            Error::SkeletonDifferentSizes(ref skeleton_name) => write!(f, "Skeleton \"{}\", has different sizes", skeleton_name),
            Error::MeshDifferentSizes(ref mesh_name) => write!(f, "Mesh \"{}\", has different sizes", mesh_name),
            Error::DuplicateBone(ref bone_name) => write!(f, "Duplicate bone \"{}\", check name of bone and skeleton", bone_name),
            Error::Other(ref message) => write!(f, "{}", message),
        }
    }
}

impl From<collada::Error> for Error {
    fn from(error:collada::Error) -> Error {
        Error::ColladaError(error)
    }
}
