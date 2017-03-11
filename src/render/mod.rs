use std;

use std::sync::Arc;
use std::sync::RwLock;
use std::sync::mpsc;

pub mod vertex;
/*
pub mod lod;
pub use self::lod::{LOD, LODTrait};

pub mod mesh;
pub use self::mesh::{Mesh, MeshTrait};

pub mod model;
pub use self::model::Model;
*/
/*
pub mod object;
pub use self::object::Object;
*/

pub mod render;
pub use self::render::{Render,RenderTask};

pub type RenderSender=mpsc::Sender<RenderTask>;

pub mod error;
pub use self::error::RenderError;

pub mod frame;
pub use self::frame::RenderFrame;
/*
pub mod frame;
pub use self::frame::ObjectFrame;

pub mod error;
pub use self::error::RenderError;

pub mod render;
pub use self::render::Render;
*/
