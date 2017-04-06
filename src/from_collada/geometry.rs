use std;
use pz5;
use collada;
use byteorder;
use cgmath;

use std::sync::Arc;
use byteorder::LittleEndian;
use byteorder::WriteBytesExt;
use std::collections::HashMap;

use super::Error;
use location::*;

pub struct Geometry{
    pub vertices_count:usize,
    pub collada_mesh:Arc<collada::Mesh>,
}

use pz5::vertex_format::VertexFormat;
use pz5::vertex_format::VertexFormatSourceLayerType;
use pz5::Pz5Geometry;

fn get_layer_by_name<'a>(source:&'a collada::Source, layer_name:&str) -> Result<&'a collada::SourceLayer, Error> {
    match source.layers.get(layer_name) {
        Some( l ) => Ok( l ),
        None => Err( Error::Other( format!("No layer of source with name \"{}\"", layer_name) )),
    }
}

macro_rules! layer_element_as_f32(
    ($layer:expr, $index:expr) => (
        match $layer{
            collada::SourceLayer::F32( ref layer_data ) => {
                layer_data[$index] as f32
            }collada::SourceLayer::I32( ref layer_data ) => {
                layer_data[$index] as f32
            },
            _ => return Err(Error::LayerMustBeI32OrF32( "F32" )),
        }
    )
);


impl Geometry{
    pub fn new(collada_mesh:Arc<collada::Mesh>) -> Geometry{
        let vertices_count=match collada_mesh.vertex_indices.iter().next(){
            Some( (_,vi) ) => vi.indices.len(),
            None => panic!("Mesh \"{}\" has no sources", collada_mesh.name),
        };

        Geometry{
            vertices_count:vertices_count,
            collada_mesh:collada_mesh,
        }
    }

    pub fn check_vertex_format(&self, vertex_format:&VertexFormat) -> Result<(),String> {
        for fvf_source in vertex_format.sources.iter(){
            let collada_vertex_indices=match self.collada_mesh.vertex_indices.get(fvf_source.name){
                Some( cvi ) => cvi,
                None => return Err( format!("No source with name\"{}\"", fvf_source.name) ),
            };

            for fvf_source_layer in fvf_source.layers.iter(){
                match collada_vertex_indices.source.layers.get(fvf_source_layer.name){
                    Some( _ ) => {},
                    None => return Err( format!("No layer of source with name \"{}\"", fvf_source_layer.name) ),
                };
            }
        }

        Ok(())
    }

    fn calculate_positions(&self, matrix:&Matrix4) -> Result<collada::VertexIndices, Error> {
        let positions_indices = match self.collada_mesh.vertex_indices.get("VERTEX") {
            Some( positions_indices ) =>
                positions_indices,
            None =>
                return Err( Error::Other(format!("No source with name\"{}\"", "VERTEX")) ),
        };

        let mut positions_x=Vec::with_capacity(positions_indices.indices.len());
        let mut positions_y=Vec::with_capacity(positions_indices.indices.len());
        let mut positions_z=Vec::with_capacity(positions_indices.indices.len());

        let x_layer=get_layer_by_name(&positions_indices.source,"X")?;
        let y_layer=get_layer_by_name(&positions_indices.source,"Y")?;
        let z_layer=get_layer_by_name(&positions_indices.source,"Z")?;

        for index in positions_indices.indices.iter() {
            let pos=cgmath::Vector4::new(
                layer_element_as_f32!(*x_layer,*index),
                layer_element_as_f32!(*y_layer,*index),
                layer_element_as_f32!(*z_layer,*index),
                1.0,
            );

            let pos=matrix*pos;

            positions_x.push( pos.x );
            positions_y.push( pos.y );
            positions_z.push( pos.z );
        }

        //TODO:Add bounding box

        let mut layers=HashMap::new();
        layers.insert(String::from("X"),collada::SourceLayer::F32(positions_x));
        layers.insert(String::from("Y"),collada::SourceLayer::F32(positions_y));
        layers.insert(String::from("Z"),collada::SourceLayer::F32(positions_z));

        let source=collada::Source{
            id:String::from("generated"),
            short_vertex_format:String::new(),
            vertex_format:String::new(),
            layers:layers,
        };

        let vertex_indices=collada::VertexIndices{
            source:Arc::new(source),
            indices:(0..positions_indices.indices.len()).collect(),
        };

        Ok(vertex_indices)
    }

    pub fn build_geometry(&self, matrix:&Matrix4, out_vertex_format:&VertexFormat) -> Result<Pz5Geometry, Error>{
        let positions=self.calculate_positions(matrix)?;

        let mut vertex_size=0;
        let mut source_map=Vec::with_capacity(3);

        for fvf_source in out_vertex_format.sources.iter(){
            let collada_vertex_indices=if fvf_source.name == "VERTEX" {
                &positions
            }else{
                match self.collada_mesh.vertex_indices.get(fvf_source.name){
                    Some( cvi ) => cvi,
                    None => return Err( Error::Other(format!("No source with name\"{}\"", fvf_source.name)) ),
                }
            };

            let mut layers=Vec::with_capacity(3);

            for fvf_source_layer in fvf_source.layers.iter(){
                let collada_source=match collada_vertex_indices.source.layers.get(fvf_source_layer.name){
                    Some( cs ) => cs,
                    None => return Err( Error::Other(format!("No layer of source with name \"{}\"", fvf_source_layer.name)) ),
                };

                vertex_size+=match fvf_source_layer.layer_type{
                    VertexFormatSourceLayerType::F32 => 4,
                    VertexFormatSourceLayerType::I32 => 4,
                };

                layers.push( (collada_source, fvf_source_layer.layer_type) );
            }

            source_map.push( (&collada_vertex_indices.indices, layers) );
        }

        let mut geometry_data:Vec<u8>=Vec::with_capacity(self.vertices_count * vertex_size);

        for i in 0..self.vertices_count {
            for source in source_map.iter(){
                let vertex_index=source.0[i];

                for layer in source.1.iter(){
                    match layer.1{
                        VertexFormatSourceLayerType::F32 => {
                            let value=match *layer.0{
                                collada::SourceLayer::F32( ref layer_data ) => {
                                    layer_data[vertex_index] as f32
                                }collada::SourceLayer::I32( ref layer_data ) => {
                                    layer_data[vertex_index] as f32
                                },
                                _ => return Err(Error::LayerMustBeI32OrF32( layer.0.print_data_type() )),
                            };

                            match geometry_data.write_f32::<LittleEndian>(value){
                                Ok ( _ ) => {},
                                Err( e ) => return Err( Error::ByteOrderError(e) ),
                            }
                        },
                        VertexFormatSourceLayerType::I32 => {
                            let value=match *layer.0{
                                collada::SourceLayer::F32( ref layer_data ) => {
                                    layer_data[vertex_index] as i32
                                }collada::SourceLayer::I32( ref layer_data ) => {
                                    layer_data[vertex_index] as i32
                                },
                                _ => return Err(Error::LayerMustBeI32OrF32( layer.0.print_data_type() )),
                            };

                            match geometry_data.write_i32::<LittleEndian>(value){
                                Ok ( _ ) => {},
                                Err( e ) => return Err( Error::ByteOrderError(e) ),
                            }
                        },
                    }
                }
            }
        }

        Ok( Pz5Geometry::from_raw(geometry_data) )
    }
}
