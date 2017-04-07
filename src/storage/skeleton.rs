use std;
use glium;
use location::*;

use std::cell::UnsafeCell;

use glium::VertexBuffer;
use glium::uniforms::UniformBuffer;

use RenderError;
use Window;
use RenderFrame;
use super::SkeletonShader;

use object_pool::growable::{ID,Slot};

#[derive(Copy,Clone)]
pub struct Vertex{
    bone_index:u32,
    color:f32,
}

implement_vertex!(Vertex, bone_index, color);

impl Vertex{
    pub fn new(bone_index:u32, color:f32) -> Self {
        Vertex{
            bone_index:bone_index,
            color:color,
            //bone_index:usize,
        }
    }
}

pub struct BonesArray{
    //bones: [f32],
    bone_matrices:[ [[f32; 4];4] ],
}

implement_buffer_content!(BonesArray);
implement_uniform_block!(BonesArray, bone_matrices);

pub struct Skeleton {
    pub id:ID,
    pub vbo:VertexBuffer<Vertex>,
    pub ubo:UnsafeCell< UniformBuffer<BonesArray> >,
}

impl Slot for Skeleton{
    fn set_id(&mut self,id:ID) {
        self.id=id;
    }

    fn get_id(&self) -> ID {
        self.id
    }
}

impl Skeleton{
    pub fn new(buffer:Vec<Vertex>, window:&Window) -> Result<Self,RenderError> {
        let ubo=UniformBuffer::empty_unsized_dynamic(&window.display, buffer.len()/2 * 16 * 4)?;
        let vbo=Self::build(buffer, window)?;

        let mut skeleton=Skeleton{
            id:ID::zeroed(),
            vbo:vbo,
            ubo:UnsafeCell::new( ubo ),
        };

        Ok(skeleton)
    }

    fn build(buffer:Vec<Vertex>, window:&Window) -> Result<VertexBuffer<Vertex>,RenderError> {
        let vbo=VertexBuffer::new(&window.display, buffer.as_ref())?;

        Ok(vbo)
    }

    fn rebuild(buffer:Vec<Vertex>, window:&Window) -> Result<VertexBuffer<Vertex>,RenderError> {
        let vbo=VertexBuffer::new(&window.display, buffer.as_ref())?;

        Ok(vbo)
    }

    pub fn load_bones(&self, bone_matrices:&[Matrix4]) {
        unsafe{
            let ubo = &mut *self.ubo.get();
            let mut mapping = ubo.map();

            for (i,m) in bone_matrices.iter().enumerate() {
                mapping.bone_matrices[i]=Into::<[[f32; 4]; 4]>::into(*m);
            }
        }
        /*
        let bones_array=BonesArray {
            bones:bones,
        };

        self.ubo.write(&bones_array);
        */
    }

    pub fn render(&self, frame:&mut RenderFrame) -> Result<(),RenderError> {
        use glium::Surface;

        let ubo=unsafe{ &*self.ubo.get() };

        let uniforms = uniform! {
            perspective_matrix: Into::<[[f32; 4]; 4]>::into(frame.perspective_matrix),
            camera_matrix: Into::<[[f32; 4]; 4]>::into(frame.camera_matrix),
            BonesArray: &*ubo,
        };

        frame.target.draw(
            &self.vbo,
            &glium::index::NoIndices(glium::index::PrimitiveType::LinesList),
            &frame.storage.skeleton_shader.glium_program,
            &uniforms,
            &frame.draw_parameters
        )?;

        Ok(())
    }
}
