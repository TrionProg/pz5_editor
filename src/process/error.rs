use std;
use pz5;
use pz5_collada;

#[derive(Debug)]
pub enum ProcessError{
    FileNameNotUTF,
    NoFileName,
    FromColladaError(Box< pz5_collada::from_collada::Error >),
    VertexFormatParseError(String),
}

impl From<pz5_collada::from_collada::Error> for ProcessError {
    fn from(from_collada_error: pz5_collada::from_collada::Error) -> Self{
        ProcessError::FromColladaError( Box::new(from_collada_error) )
    }
}

impl<'a> From<pz5::vertex_format::Error<'a>> for ProcessError {
    fn from(vertex_format_parse_error: pz5::vertex_format::Error) -> Self{
        ProcessError::VertexFormatParseError( format!("{}",vertex_format_parse_error) )
    }
}




impl std::fmt::Display for ProcessError{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self{
            ProcessError::FileNameNotUTF => write!(f, "Charset of name of file is not UTF-8"),
            ProcessError::NoFileName => write!(f, "Path to file has no name of file"),
            ProcessError::FromColladaError(ref e) => write!(f, "From collada error:{}", e),
            ProcessError::VertexFormatParseError(ref vertex_format) => write!(f, "{}",vertex_format),
            //Error::Pz5DocumentWriteError(ref e) => write!(f, "Pz5 document write error:{}", e),
            //Error::Pz5BinaryDataWriteError(ref e) => write!(f, "Pz5 document write error:{}", e),
            //Error::Other(ref message) => write!(f, "{}", message),
        }
    }
}