pub mod vertex;

pub mod lod;
pub use self::lod::{LOD, LODTrait};
/*
pub mod mesh;
pub use self::mesh::{Mesh, MeshTrait};

pub mod model;
pub use self::model::Model;

pub mod object;
pub use self::object::Object;
*/
pub mod program;
pub use self::program::Program;

pub mod frame;
pub use self::frame::ObjectFrame;

pub mod render;
pub use self::render::Render;
