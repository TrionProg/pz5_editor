use std;
use pz5;
use glium;

use object_pool::growable::ID;

#[derive(Debug)]
pub enum Error{
    //NoWindow,
    GliumCreationError(Box<glium::GliumCreationError<glium::glutin::CreationError>>),
    ProgramCreationError(Box<glium::program::ProgramCreationError>),
    ProgramChooserCreationError(Box<glium::program::ProgramChooserCreationError>),
    VertexBufferCreationError(Box<glium::vertex::BufferCreationError>),
    BufferCreationError(Box<glium::buffer::BufferCreationError>),
    NoShaderProgram(String),
    DrawError(Box<glium::DrawError>),
    NoGeometryWithID(ID),
    NoSkeletonWithID(ID),
    NoSkeleton,
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

impl From<glium::vertex::BufferCreationError> for Error {
    fn from(buffer_creation_error: glium::vertex::BufferCreationError) -> Self{
        Error::VertexBufferCreationError( Box::new(buffer_creation_error) )
    }
}

impl From<glium::buffer::BufferCreationError> for Error {
    fn from(buffer_creation_error: glium::buffer::BufferCreationError) -> Self{
        Error::BufferCreationError( Box::new(buffer_creation_error) )
    }
}

impl From<glium::DrawError> for Error {
    fn from(draw_error: glium::DrawError) -> Self{
        Error::DrawError( Box::new(draw_error) )
    }
}



impl std::fmt::Display for Error{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self{
            //Error::NoWindow => write!(f, "No window"),
            Error::GliumCreationError(ref e) => write!(f, "Can not create window. Error:{}", e),
            Error::ProgramCreationError(ref e) => write!(f, "Can not create shader program. Error:{}", e),
            Error::ProgramChooserCreationError(ref e) => write!(f, "Can not choose shader program. Error:{}", e),
            Error::NoShaderProgram(ref full_vertex_format) => write!(f, "No shader program for \"{}\"", full_vertex_format),
            Error::VertexBufferCreationError(ref e) => write!(f, "Can not create vertex buffer:{}", e),
            Error::BufferCreationError(ref e) => write!(f, "Can not create buffer:{}", e),
            Error::DrawError(ref e) => write!(f, "Can not draw:{}", e),
            Error::NoGeometryWithID(id) => write!(f, "No geometry with id {}",id),
            Error::NoSkeletonWithID(id) => write!(f, "No skeleton with id {}",id),
            Error::NoSkeleton => write!(f, "No skeleton"),
        }
    }
}
