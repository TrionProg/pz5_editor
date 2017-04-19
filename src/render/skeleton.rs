use std;
use glium;
use location::*;

use std::cell::UnsafeCell;

use glium::VertexBuffer;
use glium::uniforms::UniformBuffer;

use super::Error;
use super::Window;
use super::Frame;
use super::SkeletonShader;

use object_pool::growable::{ID,Slot};

#[derive(Copy,Clone)]
pub struct Vertex{
    pub position:[f32;3],
    pub color:f32,
    pub bone_index:u32,
}

implement_vertex!(Vertex, position, color, bone_index);

pub struct BonesArray{
    bones_matrices:[ [[f32; 4];4] ],
}

implement_buffer_content!(BonesArray);
implement_uniform_block!(BonesArray, bones_matrices);

pub struct SkeletonOfInstance {
    pub id:ID,
    pub ubo:UnsafeCell< UniformBuffer<BonesArray> >,
}

pub struct GeometryOfSkeleton {
    pub id:ID,
    pub joints_vbo:VertexBuffer<Vertex>,
    pub bones_vbo:VertexBuffer<Vertex>,
}

impl Vertex{
    pub fn new(position:Pos3D, color:f32, bone_index:u32) -> Self {
        Vertex{
            position:[position.x,position.y,position.z],
            color:color,
            bone_index:bone_index,
        }
    }
}

impl SkeletonOfInstance{
    pub fn new(bones_count:usize, window:&Window) -> Result<Self,Error> {
        let ubo=UniformBuffer::empty_unsized_dynamic(&window.display, bones_count * 16 * 4)?;

        let mut skeleton_of_instance=SkeletonOfInstance{
            id:ID::zeroed(),
            ubo:UnsafeCell::new( ubo ),
        };

        Ok(skeleton_of_instance)
    }

    /*
    fn build(buffer:Vec<Vertex>, window:&Window) -> Result<VertexBuffer<Vertex>,Error> {
        let vbo=VertexBuffer::new(&window.display, buffer.as_ref())?;

        Ok(vbo)
    }

    fn rebuild(buffer:Vec<Vertex>, window:&Window) -> Result<VertexBuffer<Vertex>,Error> {
        let vbo=VertexBuffer::new(&window.display, buffer.as_ref())?;

        Ok(vbo)
    }
    */

    pub fn load_bones(&self, bones_matrices:&[Matrix4]) {
        unsafe{
            let ubo = &mut *self.ubo.get();
            let mut mapping = ubo.map();

            for (i,m) in bones_matrices.iter().enumerate() {
                mapping.bones_matrices[i]=Into::<[[f32; 4]; 4]>::into(*m);
            }
        }
    }
}

impl Slot for SkeletonOfInstance{
    fn set_id(&mut self,id:ID) {
        self.id=id;
    }

    fn get_id(&self) -> ID {
        self.id
    }
}



impl GeometryOfSkeleton{
    pub fn new(joints_buffer:Vec<Vertex>, bones_buffer:Vec<Vertex>, window:&Window) -> Result<Self,Error> {
        let joints_vbo=VertexBuffer::new(&window.display, joints_buffer.as_ref())?;
        let bones_vbo=VertexBuffer::new(&window.display, bones_buffer.as_ref())?;

        let mut geometry_of_skeleton=GeometryOfSkeleton{
            id:ID::zeroed(),
            joints_vbo:joints_vbo,
            bones_vbo:bones_vbo,
        };

        Ok(geometry_of_skeleton)
    }

    /*
    fn build(buffer:Vec<Vertex>, window:&Window) -> Result<VertexBuffer<Vertex>,Error> {
        let vbo=VertexBuffer::new(&window.display, buffer.as_ref())?;

        Ok(vbo)
    }

    fn rebuild(buffer:Vec<Vertex>, window:&Window) -> Result<VertexBuffer<Vertex>,Error> {
        let vbo=VertexBuffer::new(&window.display, buffer.as_ref())?;

        Ok(vbo)
    }
    */

    pub fn render(&self, frame:&mut Frame, skeleton_of_instance:&SkeletonOfInstance) -> Result<(),Error> {
        use glium::Surface;

        let ubo=unsafe{ &*skeleton_of_instance.ubo.get() };

        let uniforms = uniform! {
            perspective_matrix: Into::<[[f32; 4]; 4]>::into(frame.perspective_matrix),
            camera_matrix: Into::<[[f32; 4]; 4]>::into(frame.camera_matrix),
            BonesArray: &*ubo,
        };

        frame.target.draw(
            &self.bones_vbo,
            &glium::index::NoIndices(glium::index::PrimitiveType::LinesList),
            &frame.storage.skeleton_shader.glium_program,
            &uniforms,
            &frame.draw_parameters
        )?;

        frame.target.draw(
            &self.joints_vbo,
            &glium::index::NoIndices(glium::index::PrimitiveType::Points),
            &frame.storage.skeleton_shader.glium_program,
            &uniforms,
            &frame.draw_parameters
        )?;

        Ok(())
    }
}

impl Slot for GeometryOfSkeleton{
    fn set_id(&mut self,id:ID) {
        self.id=id;
    }

    fn get_id(&self) -> ID {
        self.id
    }
}
