use std;
use pz5;
use pz5_collada;
use render;

use std::rc::Rc;
use pz5_collada::from_collada::FromColladaLOD;

use Error;
use Render;
use super::Geometry;

pub struct LOD{
    key_distance:f32,

    pub id:usize,
    pub distance:f32,
    pub geometry:Geometry,
    pub vertices_count:usize,
    pub description:String,

    pub include:bool,
    pub display:bool,

    render_lod:Option<Box<render::LODTrait>>,
}

impl FromColladaLOD for LOD{
    type Error=Error;
}

impl LOD{
    pub fn new(
        distance:f32,
        id:usize,
        geometry:Geometry,
        vertices_count:usize,
        description:String,
        render:Option<Rc<Render>>,
        vertex_format:&String,
    ) -> Result<Self,Error> {
        let key_distance=distance.clone();

        let display=render.is_some();

        let mut lod=LOD{
            key_distance:key_distance,

            id:id,
            distance:distance,
            geometry:geometry,
            vertices_count:vertices_count,
            description:description,

            include:true,
            display:display,

            render_lod:None,
        };

        //TODO:load render_lod

        Ok(lod)
    }

    //load render_lod fn
}
