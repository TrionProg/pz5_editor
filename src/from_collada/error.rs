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
    NoSkeleton(String),
    BoneDifferentSizes(String,String),
    SkeletonDifferentSizes(String),
    MeshDifferentSizes(String),
    DuplicateBone(String),
    MeshLODLocationsMismatch(String,String),
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
            Error::NoSkeleton(ref skeleton_id) => write!(f, "Skeleton with id \"{}\" does not exists", skeleton_id),
            Error::BoneDifferentSizes(ref skeleton_name, ref bone_name) => write!(f, "Bone \"{}\" of skeleton \"{}\", has different sizes", bone_name, skeleton_name),
            Error::SkeletonDifferentSizes(ref skeleton_name) => write!(f, "Skeleton \"{}\", has different sizes", skeleton_name),
            Error::MeshDifferentSizes(ref mesh_name) => write!(f, "Mesh \"{}\", has different sizes", mesh_name),
            Error::DuplicateBone(ref bone_name) => write!(f, "Duplicate bone \"{}\", check name of bone and skeleton", bone_name),
            Error::MeshLODLocationsMismatch(ref mesh_name, ref lod_name) => write!(f, "Location of lod \"{}\" mismatch with location of mesh \"{}\"", lod_name, mesh_name),
            Error::Other(ref message) => write!(f, "{}", message),
        }
    }
}

impl From<collada::Error> for Error {
    fn from(error:collada::Error) -> Error {
        Error::ColladaError(error)
    }
}
