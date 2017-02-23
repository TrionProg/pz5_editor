use std;
use pz5;
use pz5_collada;
use collada;

use super::Mesh;

enum Parent<'a>{
    ColladaDocument(collada::ColladaDocument, Vec<virtualMeshes>),
    //Pz5Model(&'a Model),
}

pub struct Model<'a>{
    parent:Parent<'a>,
    pub name:String,
    pub meshes:Vec<Mesh>,
}
/*
impl ToPz5Model for Model{
    type Mesh=Mesh;

    fn get_name(&self) -> &String{
        &self.name
    }

    fn get_meshes(&self) -> &HashMap<String, Self::Mesh>{
        &self.meshes
    }
    /*
    fn write<WriteTo:std::io::Write>(&self,write_to:&mut WriteTo) -> Result<(),pz5::Error>{
        Ok(())
    }


    fn read<ReadFrom:std::io::Read>(read_from:&ReadFrom) -> Result<Self,pz5::Error>{
        Ok(Model{name:String::from("hello"),meshes:HashMap::new()})
    }
    */

    fn print(&self){
        println!("Model {}",self.name);
        for (_,mesh) in self.meshes.iter(){
            mesh.print();
        }
    }
}
*/

/*
impl FromColladaModel for Model{
    type Mesh=Mesh;
    type Error=pz5_collada::Error;
}

impl Model{
    pub fn load_from_collada(file_name:&Path) -> Result<Model, pz5_collada::Error>{
        let model=Self::read_collada(file_name,
        |document, virtual_meshes|{
            let meshes=Self::build_meshes(virtual_meshes,
            |virtual_mesh|{
                let lods=Mesh::build_lods(virtual_mesh,
                |virtual_lod,geometry|{

                    Ok(
                        LOD{
                            distance:virtual_lod.distance,
                            geometry:geometry,
                            vertices_count:virtual_lod.vertices_count,
                        }
                    )
                })?;

                Ok(
                    Mesh{
                        name:virtual_mesh.name.clone(),
                        semantics:virtual_mesh.full_semantics.clone(),
                        geometry_type:virtual_mesh.geometry_type,
                        lods:lods,
                    }
                )
            })?;

            Ok(
                Model{
                    name:String::from("wow"),
                    meshes:meshes,
                }
            )
        })?;

        Ok(model)
    }
}
*/
