use std;
use cgmath;
use render;

use cgmath::{Rad, PerspectiveFov,Matrix4};
use cgmath::rad;

use render::Window;

pub struct Viewport{
    pub width:u32,
    pub height:u32,
    pub perspective_matrix:Matrix4<f32>,
}

impl Viewport{
    pub fn configure(window:&render::Window) -> Option<Self>{
        //TODO:none if width<0
        let width=window.width;
        let height=window.height;

        let aspect_ratio=width as f32 / height as f32;

        let perspective=PerspectiveFov{
            fovy:rad(0.5),
            aspect:aspect_ratio,
            near:0.1,
            far:1000.0,
        };

        let perspective_matrix=Matrix4::from(perspective);

        Some(
            Viewport{
                width:width,
                height:height,
                perspective_matrix:perspective_matrix,
            }
        )
    }
}
