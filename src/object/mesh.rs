use std;
use pz5;
use pz5_collada;
use glium;
use render;

use std::rc::Rc;

use pz5_collada::from_collada::FromColladaMesh;
use std::collections::HashMap;

use Error;
use Render;

use super::LOD;
use ObjectFrame;


pub struct Mesh{
    key_name:String,
    in_full_vertex_format:String,

    pub name:String,
    pub full_vertex_format:String,
    pub geometry_type:pz5::GeometryType,
    pub lods:Vec<Rc<LOD>>,
    pub description:String,

    pub include:bool,
    pub display:bool,
}

impl FromColladaMesh for Mesh{
    type LOD=LOD;
    type Container=Rc<LOD>;
    type Error=Error;

    fn get_name(&self) -> &String{
        &self.name
    }
}

impl Mesh{
    pub fn new(
        name:String,
        in_full_vertex_format:String,
        full_vertex_format:String,
        geometry_type:pz5::GeometryType,
        lods:Vec<Rc<LOD>>,
        description:String,
        render:Option<Rc<Render>>,
    ) -> Result<Self, Error>{
        let key_name=name.clone();

        let mut mesh=Mesh{
            key_name:key_name,
            in_full_vertex_format:in_full_vertex_format,

            name:name,
            full_vertex_format:full_vertex_format,
            geometry_type:geometry_type,
            lods:lods,
            description:description,

            include:true,
            display:render.is_some(),
        };

        match render{
            Some( ref render ) => mesh.build_render_lods(render)?,
            None => {},
        }

        Ok( mesh )
    }

    pub fn build_render_lods(&mut self, render:&Render) -> Result<(),Error> {
        let full_vertex_format_str=String::from("VERTEX:(X:float,Y:float)");
        let full_vertex_format=pz5::VertexFormat::parse(&full_vertex_format_str).unwrap();

        for lod in self.lods.iter_mut(){
            Rc::get_mut(lod).unwrap().build_render_lod(render, &full_vertex_format, &full_vertex_format_str, self.geometry_type)?;
        }

        Ok(())
    }

    pub fn render(&self, frame:&mut ObjectFrame) -> Result<(),glium::DrawError>{
        if !self.display {
            return Ok(());
        }

        self.lods[0].render(frame)?;

        Ok(())
    }

    pub fn adapt_vertex_format(in_fvf:&String) -> Result<String,Error> {
        Ok( in_fvf.clone() )
    }
}
