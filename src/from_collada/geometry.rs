use std;
use pz5;
use collada;
use byteorder;

use std::sync::Arc;
use byteorder::LittleEndian;
use byteorder::WriteBytesExt;

use super::Error;

pub struct Geometry{
    pub vertices_count:usize,
    pub collada_mesh:Arc<collada::Mesh>,
}

use pz5::vertex_format::VertexFormat;
use pz5::vertex_format::VertexFormatSourceLayerType;
use pz5::Pz5Geometry;

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

    pub fn build_geometry(&self, out_vertex_format:&VertexFormat) -> Result<Pz5Geometry, Error>{
        let mut vertex_size=0;
        let mut source_map=Vec::with_capacity(3);

        for fvf_source in out_vertex_format.sources.iter(){
            let collada_vertex_indices=match self.collada_mesh.vertex_indices.get(fvf_source.name){
                Some( cvi ) => cvi,
                None => return Err( Error::Other(format!("No source with name\"{}\"", fvf_source.name)) ),
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
