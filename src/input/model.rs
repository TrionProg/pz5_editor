use std;
use pz5;
use pz5_collada;

use std::rc::Rc;

use pz5_collada::from_collada::FromColladaModel;
use pz5_collada::from_collada::FromColladaMesh;

use std::collections::HashMap;
use std::path::Path;

use Error;

use super::Mesh;
use super::LOD;
use super::Geometry;

pub struct Model{
    pub name:String,
    pub meshes:HashMap<String,Rc<Mesh>>,
}

impl FromColladaModel for Model{
    type Mesh=Mesh;
    type Container=Rc<Mesh>;
    type Error=Error;
}

impl Model{
    pub fn load_from_collada(file_name:&Path) -> Result<Model, Error>{
        let model_name=match file_name.file_name() {
            Some( file_name_os_str ) => {
                match file_name_os_str.to_str() {
                    Some( file_name_str ) => String::from(file_name_str),
                    None => return Err( Error::FileNameNotUTF ),
                }
            },
            None => return Err( Error::NoFileName ),
        };

        let model=<Self as FromColladaModel>::build(file_name,
        |document, virtual_meshes|{
            let meshes=Self::build_meshes(virtual_meshes,
            |virtual_mesh|{
                let lods=Mesh::build_lods(virtual_mesh,
                |virtual_lod,collada_geometry|{
                    let id=collada_geometry.id;

                    Ok(
                        LOD{
                            id:id,
                            distance:virtual_lod.distance,
                            geometry:Geometry::ColladaGeometry(collada_geometry),
                            vertices_count:virtual_lod.vertices_count,
                        }
                    )
                })?;

                Ok(
                    Mesh{
                        name:virtual_mesh.name.clone(),
                        vertex_full_format:virtual_mesh.full_vertex_format.clone(),
                        geometry_type:virtual_mesh.geometry_type,
                        lods:lods,
                    }
                )
            })?;

            Ok(
                Model{
                    name:model_name,
                    meshes:meshes,
                }
            )
        })?;

        Ok(model)
    }
}
