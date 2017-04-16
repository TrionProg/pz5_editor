use std;
use glium;

use glium::VertexBuffer;

use super::Error;
use super::Window;
use super::Frame;
use super::GridShader;

#[derive(Copy,Clone)]
pub struct Vertex{
    position:[f32;2],
}
implement_vertex!(Vertex, position);

impl Vertex{
    pub fn new(x:f32,y:f32) -> Self {
        Vertex{
            position:[x,y],
        }
    }
}

pub struct Grid{
    pub vbo:VertexBuffer<Vertex>,
}

impl Grid{
    pub fn new(distance:f32, window:&Window) -> Result<Self,Error> {
        let vbo=Self::build(distance, window)?;

        let mut grid=Grid{
            vbo:vbo,
        };

        Ok(grid)
    }

    fn build(distance:f32, window:&Window) -> Result<VertexBuffer<Vertex>,Error> {
        let mut distance_ceil=if distance>200.0 {
            200.0
        }else if distance<1.0 {
            1.0
        }else{
            distance.ceil()
        };

        let buffer_size=distance_ceil as usize*4;
        let mut buffer=Vec::with_capacity(buffer_size);

        for i in (-distance_ceil as i32) .. (distance_ceil as i32 + 1) {
            let x=i as f32;
            buffer.push(Vertex::new(x, -distance_ceil));
            buffer.push(Vertex::new(x,  distance_ceil));

            let y=i as f32;
            buffer.push(Vertex::new(-distance_ceil, y));
            buffer.push(Vertex::new( distance_ceil, y));
        }

        let vbo=VertexBuffer::new(&window.display, buffer.as_ref())?;

        Ok(vbo)
    }

    pub fn rebuild(&mut self, distance:f32, window:&Window) -> Result<(),Error> {
        self.vbo=Self::build(distance, window)?;

        Ok(())
    }

    pub fn render(&self, frame:&mut Frame, grid_shader:&GridShader) -> Result<(),Error> {
        use glium::Surface;

        let uniforms = uniform! {
            perspective_matrix: Into::<[[f32; 4]; 4]>::into(frame.perspective_matrix),
            camera_matrix: Into::<[[f32; 4]; 4]>::into(frame.camera_matrix),
        };

        frame.target.draw(
            &self.vbo,
            &glium::index::NoIndices(glium::index::PrimitiveType::LinesList),
            &grid_shader.glium_program,
            &uniforms,
            &frame.draw_parameters
        )?;

        Ok(())
    }
}
