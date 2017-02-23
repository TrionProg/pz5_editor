use std;
use pz5;
use pz5_collada;
use glium;

#[derive(Debug)]
pub enum Error{
    FileNameNotUTF,
    NoFileName,
    FromColladaError(Box< pz5_collada::from_collada::Error >),
    GliumCreationError(Box<glium::GliumCreationError<glium::glutin::CreationError>>),
    ProgramCreationError(Box<glium::program::ProgramCreationError>),
    ProgramChooserCreationError(Box<glium::program::ProgramChooserCreationError>),
}

impl From<pz5_collada::from_collada::Error> for Error {
    fn from(from_collada_error: pz5_collada::from_collada::Error) -> Self{
        Error::FromColladaError( Box::new(from_collada_error) )
    }
}

impl From<glium::GliumCreationError<glium::glutin::CreationError>> for Error {
    fn from(glium_creation_error: glium::GliumCreationError<glium::glutin::CreationError>) -> Self{
        Error::GliumCreationError( Box::new(glium_creation_error) )
    }
}

impl From<glium::program::ProgramCreationError> for Error {
    fn from(program_creation_error: glium::program::ProgramCreationError) -> Self{
        Error::ProgramCreationError( Box::new(program_creation_error) )
    }
}

impl From<glium::program::ProgramChooserCreationError> for Error {
    fn from(program_chooser_creation_error: glium::program::ProgramChooserCreationError) -> Self{
        Error::ProgramChooserCreationError( Box::new(program_chooser_creation_error) )
    }
}

impl std::fmt::Display for Error{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self{
            Error::FileNameNotUTF => write!(f, "Charset of name of file is not UTF-8"),
            Error::NoFileName => write!(f, "Path to file has no name of file"),
            Error::FromColladaError(ref e) => write!(f, "From collada error:{}", e),
            Error::GliumCreationError(ref e) => write!(f, "Can not create window. Error:{}", e),
            Error::ProgramCreationError(ref e) => write!(f, "Can not create shader program. Error:{}", e),
            Error::ProgramChooserCreationError(ref e) => write!(f, "Can not choose shader program. Error:{}", e),
            //Error::Pz5DocumentWriteError(ref e) => write!(f, "Pz5 document write error:{}", e),
            //Error::Pz5BinaryDataWriteError(ref e) => write!(f, "Pz5 document write error:{}", e),
            //Error::Other(ref message) => write!(f, "{}", message),
        }
    }
}
