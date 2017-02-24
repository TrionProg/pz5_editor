use std;
use pz5;
use pz5_collada;
use glium;
use render;

use std::rc::Rc;
use pz5_collada::from_collada::FromColladaLOD;
use render::LODTrait;

use Error;
use Render;
use super::Geometry;
use ObjectFrame;

pub struct LOD{
    key_distance:f32,

    pub id:usize,
    pub distance:f32,
    pub geometry:Geometry,
    pub vertices_count:usize,
    pub description:String,

    pub include:bool,
    pub display:bool,

    render_lod:Option<Box<LODTrait>>,
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
        display:bool,
    ) -> Result<Self,Error> {
        let key_distance=distance.clone();

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

        Ok(lod)
    }

    pub fn build_render_lod(&mut self, render:&Render, fvf:&pz5::VertexFormat, fvf_str:&String, geometry_type:pz5::GeometryType) -> Result<(),Error> {
        let render_lod=self.geometry.build_render_lod(render, fvf, fvf_str, geometry_type)?;

        self.render_lod=Some(render_lod);

        Ok(())
    }

    pub fn render(&self, frame:&mut ObjectFrame) -> Result<(),glium::DrawError>{
        if !self.display {
            return Ok(());
        }

        match self.render_lod{
            Some( ref render_lod ) => render_lod.render( frame ),
            None => Ok(())
        }
    }

}
