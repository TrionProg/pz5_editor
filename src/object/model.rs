use std;
use pz5;
use pz5_collada;

use std::rc::Rc;

use pz5_collada::from_collada::FromColladaModel;
use pz5_collada::from_collada::FromColladaMesh;

use std::collections::HashMap;
use std::path::Path;

use Error;
use Render;

use super::Mesh;
use super::LOD;
use super::Geometry;

pub struct Model{
    key_id:usize,

    pub name:String,
    pub meshes:HashMap<String,Rc<Mesh>>,
    pub description:String,

    pub include:bool,
    pub display:bool,
}

impl FromColladaModel for Model{
    type Mesh=Mesh;
    type Container=Rc<Mesh>;
    type Error=Error;
}

impl Model{
    pub fn new(
        key_id:usize,
        name:String,
        meshes:HashMap<String,Rc<Mesh>>,
        description:String,
        display:bool,
    ) -> Result<Self, Error>{
        let model=Model{
            key_id:key_id,

            name:name,
            meshes:meshes,
            description:description,

            include:true,
            display:display,
        };

        Ok( model )
    }

    fn get_model_name(file_name:&Path) -> Result<String,Error>{
        let model_name=match file_name.file_name() {
            Some( file_name_os_str ) => {
                match file_name_os_str.to_str() {
                    Some( file_name_str ) => String::from(file_name_str),//TODO:remove extension
                    None => return Err( Error::FileNameNotUTF ),
                }
            },
            None => return Err( Error::NoFileName ),
        };

        Ok(model_name)
    }

    pub fn include_collada_model(file_name:&Path, model_id:usize, render:&Option<Rc<Render>>) -> Result<Self,Error> {
        let model_name=Self::get_model_name(file_name)?;

        let model=<Self as FromColladaModel>::build(file_name,
        |document, virtual_meshes|{
            let meshes=Self::build_meshes(virtual_meshes,
            |virtual_mesh|{
                let vertex_format=Mesh::adapt_vertex_format(&virtual_mesh.full_vertex_format)?;

                let lods=Mesh::build_lods(virtual_mesh,
                |virtual_lod,collada_geometry|{
                    let id=collada_geometry.id;

                    LOD::new(
                        virtual_lod.distance,
                        id,
                        Geometry::ColladaGeometry(collada_geometry),
                        virtual_lod.vertices_count,
                        String::new(),
                        render.clone(),
                        &vertex_format
                    )
                })?;

                Mesh::new(
                    virtual_mesh.name.clone(),
                    virtual_mesh.full_vertex_format.clone(),
                    vertex_format,
                    virtual_mesh.geometry_type,
                    lods,
                    String::new(),
                    render.is_some(),
                )
            })?;

            Model::new(
                model_id,
                model_name,
                meshes,
                String::new(),
                render.is_some(),
            )
        })?;

        Ok(model)
    }
}
