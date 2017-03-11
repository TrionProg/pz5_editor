use std;
use glium;

use glium::VertexBuffer;

use RenderError;
use Window;
use RenderFrame;
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
    pub fn new(distance:f32, window:&Window) -> Result<Self,RenderError> {
        let vbo=Self::build(distance, window)?;

        let mut grid=Grid{
            vbo:vbo,
        };

        Ok(grid)
    }

    fn build(distance:f32, window:&Window) -> Result<VertexBuffer<Vertex>,RenderError> {
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

    pub fn rebuild(&mut self, distance:f32, window:&Window) -> Result<(),RenderError> {
        self.vbo=Self::build(distance, window)?;

        Ok(())
    }

    /*
    pub fn render(&self, frame:&RenderFrame, grid_shader:&GridShader)
        frame.target.draw(&self.vbo,
            &glium::index::NoIndices(glium::index::PrimitiveType::LinesList),
            &grid_shader.glium_program, &uniforms, &frame.draw_parameters).unwrap();
    */
}

/*
pub struct Grid{
    vbo:VertexBuffer<Vertex>,
    buffer_size:usize,
}

impl Grid{
    pub fn new(distance:f32, window:&Window) -> Result<Self,Error> {
        let buffer_size=40*4;
        let vbo=VertexBuffer::empty_dynamic(&window.display,buffer_size)?;

        let mut grid=Grid{
            vbo:vbo,
            buffer_size:buffer_size,
        };

        grid.build(distance, window) ?;

        Ok(grid)
    }

    fn build(&mut self, distance:f32, window:&Window) -> Result<(),Error> {
        let mut distance_ceil=if distance>200.0 {
            200.0
        }else if distance<1.0 {
            1.0
        }else{
            distance.ceil()
        };

        let buffer_size=distance_ceil as usize*4;
        let mut buffer=Vec::with_capacity(buffer_size);

        if buffer_size>self.buffer_size {
            let mut new_buffer_size=self.buffer_size;
            while new_buffer_size<buffer_size {
                new_buffer_size*=2;
            }

            self.buffer_size=new_buffer_size;
            self.vbo=VertexBuffer::empty_dynamic(&window.display,new_buffer_size)?;
        }else if buffer_size<self.buffer_size/2 {
            let mut new_buffer_size=self.buffer_size;
            while new_buffer_size/2>buffer_size {
                new_buffer_size/=2;
            }

            println!("BS:{} {}",new_buffer_size, buffer_size);

            self.buffer_size=new_buffer_size;
            self.vbo=VertexBuffer::empty_dynamic(&window.display,new_buffer_size)?;
        }

        for i in (-distance_ceil as i32) .. (distance_ceil as i32 + 1) {
            let x=i as f32;
            buffer.push(Vertex::new(x, -distance_ceil));
            buffer.push(Vertex::new(x,  distance_ceil));

            let y=i as f32;
            buffer.push(Vertex::new(-distance_ceil, y));
            buffer.push(Vertex::new( distance_ceil, y));

            println!("{}",i);
        }

        for

        self.vbo.write(buffer.as_ref());

        Ok(())
    }

    /*
    fn build(&mut self, distance:f32, window:&Window) -> Result<(),Error> {
        let mut distance_ceil=if distance>200.0 {
            200.0
        }else if distance<1.0 {
            1.0
        }else{
            distance.ceil()
        };

        let buffer_size=distance_ceil as usize*4;
        let mut buffer=Vec::with_capacity(buffer_size);

        if buffer_size>self.buffer_size {
            let mut new_buffer_size=self.buffer_size;
            while new_buffer_size<buffer_size {
                new_buffer_size*=2;
            }

            self.buffer_size=new_buffer_size;
            self.vbo=VertexBuffer::empty_dynamic(&window.display,new_buffer_size)?;
        }else if buffer_size<self.buffer_size/2 {
            let mut new_buffer_size=self.buffer_size;
            while new_buffer_size/2>buffer_size {
                new_buffer_size/=2;
            }

            println!("BS:{} {}",new_buffer_size, buffer_size);

            self.buffer_size=new_buffer_size;
            self.vbo=VertexBuffer::empty_dynamic(&window.display,new_buffer_size)?;
        }

        for i in (-distance_ceil as i32) .. (distance_ceil as i32 + 1) {
            let x=i as f32;
            buffer.push(Vertex::new(x, -distance_ceil));
            buffer.push(Vertex::new(x,  distance_ceil));

            let y=i as f32;
            buffer.push(Vertex::new(-distance_ceil, y));
            buffer.push(Vertex::new( distance_ceil, y));

            println!("{}",i);
        }

        for

        self.vbo.write(buffer.as_ref());

        Ok(())
    }
    */

    pub fn rebuild(&mut self, distance:f32, window:&Window) -> Result<(),Error> {
        self.build(distance, window)
    }
}
*/
