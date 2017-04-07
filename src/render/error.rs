use std;
use pz5;
use glium;

use object_pool::growable::ID;

#[derive(Debug)]
pub enum RenderError{
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


impl From<glium::GliumCreationError<glium::glutin::CreationError>> for RenderError {
    fn from(glium_creation_error: glium::GliumCreationError<glium::glutin::CreationError>) -> Self{
        RenderError::GliumCreationError( Box::new(glium_creation_error) )
    }
}

impl From<glium::program::ProgramCreationError> for RenderError {
    fn from(program_creation_error: glium::program::ProgramCreationError) -> Self{
        RenderError::ProgramCreationError( Box::new(program_creation_error) )
    }
}

impl From<glium::program::ProgramChooserCreationError> for RenderError {
    fn from(program_chooser_creation_error: glium::program::ProgramChooserCreationError) -> Self{
        RenderError::ProgramChooserCreationError( Box::new(program_chooser_creation_error) )
    }
}

impl From<glium::vertex::BufferCreationError> for RenderError {
    fn from(buffer_creation_error: glium::vertex::BufferCreationError) -> Self{
        RenderError::VertexBufferCreationError( Box::new(buffer_creation_error) )
    }
}

impl From<glium::buffer::BufferCreationError> for RenderError {
    fn from(buffer_creation_error: glium::buffer::BufferCreationError) -> Self{
        RenderError::BufferCreationError( Box::new(buffer_creation_error) )
    }
}

impl From<glium::DrawError> for RenderError {
    fn from(draw_error: glium::DrawError) -> Self{
        RenderError::DrawError( Box::new(draw_error) )
    }
}



impl std::fmt::Display for RenderError{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self{
            //RenderError::NoWindow => write!(f, "No window"),
            RenderError::GliumCreationError(ref e) => write!(f, "Can not create window. Error:{}", e),
            RenderError::ProgramCreationError(ref e) => write!(f, "Can not create shader program. Error:{}", e),
            RenderError::ProgramChooserCreationError(ref e) => write!(f, "Can not choose shader program. Error:{}", e),
            RenderError::NoShaderProgram(ref full_vertex_format) => write!(f, "No shader program for \"{}\"", full_vertex_format),
            RenderError::VertexBufferCreationError(ref e) => write!(f, "Can not create vertex buffer:{}", e),
            RenderError::BufferCreationError(ref e) => write!(f, "Can not create buffer:{}", e),
            RenderError::DrawError(ref e) => write!(f, "Can not draw:{}", e),
            RenderError::NoGeometryWithID(id) => write!(f, "No geometry with id {}",id),
            RenderError::NoSkeletonWithID(id) => write!(f, "No skeleton with id {}",id),
            RenderError::NoSkeleton => write!(f, "No skeleton"),
        }
    }
}
