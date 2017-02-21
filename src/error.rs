use std;
use pz5;
use pz5_collada;

#[derive(Debug)]
pub enum Error{
    FileNameNotUTF,
    NoFileName,
    FromColladaError(Box< pz5_collada::from_collada::Error >),
}

impl From<pz5_collada::from_collada::Error> for Error {
    fn from(from_collada_error: pz5_collada::from_collada::Error) -> Self{
        Error::FromColladaError( Box::new(from_collada_error) )
    }
}

impl std::fmt::Display for Error{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self{
            Error::FileNameNotUTF => write!(f, "Charset of name of file is not UTF-8"),
            Error::NoFileName => write!(f, "Path to file has no name of file"),
            Error::FromColladaError(ref e) => write!(f, "From collada error:{}", e),
            //Error::Pz5DocumentWriteError(ref e) => write!(f, "Pz5 document write error:{}", e),
            //Error::Pz5BinaryDataWriteError(ref e) => write!(f, "Pz5 document write error:{}", e),
            //Error::Other(ref message) => write!(f, "{}", message),
        }
    }
}
