use std;
use pz5;
use from_collada;
use render;

use std::sync::mpsc::SendError;

#[derive(Debug)]
pub enum Error{
    FileNameNotUTF,
    NoFileName,
    FromColladaError(Box< from_collada::Error >),
    VertexFormatParseError(String),
    SendTaskError(Box<SendError<render::Task>>),
    DuplicateInstance(String),
    DuplicateBone(String),
    SkeletonHasSeparatedBranches(String),
}

impl From<from_collada::Error> for Error {
    fn from(from_collada_error: from_collada::Error) -> Self{
        Error::FromColladaError( Box::new(from_collada_error) )
    }
}

impl<'a> From<pz5::vertex_format::Error<'a>> for Error {
    fn from(vertex_format_parse_error: pz5::vertex_format::Error) -> Self{
        Error::VertexFormatParseError( format!("{}",vertex_format_parse_error) )
    }
}

impl<'a> From<SendError<render::Task>> for Error {
    fn from(send_task_error: SendError<render::Task>) -> Self{
        Error::SendTaskError( Box::new(send_task_error) )
    }
}




impl std::fmt::Display for Error{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self{
            Error::FileNameNotUTF => write!(f, "Charset of name of file is not UTF-8"),
            Error::NoFileName => write!(f, "Path to file has no name of file"),
            Error::FromColladaError(ref e) => write!(f, "From collada error:{}", e),
            Error::VertexFormatParseError(ref vertex_format) => write!(f, "{}",vertex_format),
            Error::SendTaskError(ref e) => write!(f, "Can not send error:{}",e),
            Error::DuplicateInstance(ref instance_name) => write!(f, "Duplicate instance \"{}\"", instance_name),
            Error::DuplicateBone(ref bone_name) => write!(f, "Duplicate bone \"{}\"", bone_name),
            Error::SkeletonHasSeparatedBranches(ref skeleton_name) => write!(f, "Skeleton \"{}\" has separated branches", skeleton_name),
            //Error::Pz5DocumentWriteError(ref e) => write!(f, "Pz5 document write error:{}", e),
            //Error::Pz5BinaryDataWriteError(ref e) => write!(f, "Pz5 document write error:{}", e),
            //Error::Other(ref message) => write!(f, "{}", message),
        }
    }
}
