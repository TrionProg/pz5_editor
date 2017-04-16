use Error;
use pz5;
use collada;

pub struct VirtualSource<'a>{
    pub vertex_layer:&'a collada::VertexLayer,
    pub is_index:bool,
    pub layers:Vec<VirtualSourceLayer<'a>>,
}

pub struct VirtualSourceLayer<'a>{
    pub layer:&'a collada::SourceLayer,
    pub layer_type:pz5::SemanticsSourceLayerType,
}

impl<'a> VirtualSource<'a>{
    pub fn construct(collada_mesh:&'a collada::Mesh, vertex_format_source:&pz5::SemanticsSource) -> Result<VirtualSource<'a>,Error>{
        let vertex_layer=match collada_mesh.vertex_layers.get(vertex_format_source.name){
            Some( vertex_layer_rc ) => vertex_layer_rc,
            None => return Err( Error::Other(format!("Mesh has no vertex layer \"{}\"", vertex_format_source.name)) ),
        };

        let mut layers=Vec::with_capacity(vertex_format_source.layers.len());

        for vertex_format_source_layer in vertex_format_source.layers.iter(){
            let source_layer=match vertex_layer.source.layers.get(vertex_format_source_layer.name){
                Some( source_layer ) => source_layer,
                None => return Err( Error::Other(format!("Source \"{}\" has no layer \"{}\"", &vertex_layer.source.id, vertex_format_source_layer.name)) ),
            };

            layers.push(VirtualSourceLayer{
                layer:source_layer,
                layer_type:vertex_format_source_layer.layer_type,
            });
        }

        Ok(VirtualSource{
            vertex_layer:vertex_layer,
            is_index:vertex_format_source.is_index,
            layers:layers,
        })
    }
}
