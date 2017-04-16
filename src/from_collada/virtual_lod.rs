use std;
use pz5;
use collada;

use std::sync::Arc;

use super::Error;

pub struct VirtualLOD<'a>{
    pub distance:f32,
    pub geometry_type:pz5::GeometryType,
    pub geometry:&'a Arc<collada::Mesh>,
    pub vertices_count:usize,
}

impl<'a> VirtualLOD<'a>{
    pub fn construct(collada_mesh:&'a Arc<collada::Mesh>, distance:f32) -> Result<VirtualLOD<'a>,Error>{
        let vertices_count=match collada_mesh.vertex_indices.iter().next(){
            Some( (_,vertex_indices) ) =>
                vertex_indices.indices.len(),
            None => return Err( Error::NoVertices ),
        };

        let geometry_type=Self::get_geometry_type(collada_mesh)?;

        Ok(
            VirtualLOD{
                distance:distance,
                geometry_type:geometry_type,
                geometry:collada_mesh,
                vertices_count:vertices_count,
            }
        )
    }

    fn get_geometry_type(collada_mesh:&collada::Mesh) -> Result<pz5::GeometryType,Error>{
        if collada_mesh.polygons.len()==0 {
            return Err( Error::NoPolygons );
        }

        let vertex_count_per_polygon=collada_mesh.polygons[0].vertices_count;

        for polygon in collada_mesh.polygons.iter().skip(1){
            if polygon.vertices_count!=vertex_count_per_polygon {
                return Err( Error::Other(format!("Mesh expects {} vertices per polygon, but polygon with {} vertices has been found", vertex_count_per_polygon, polygon.vertices_count)) );
            }
        }

        match pz5::GeometryType::from_vertices_count(vertex_count_per_polygon){
            Ok( gt ) => Ok(gt),
            Err( e ) => Err( Error::Other(e) ),
        }
    }
}
