use std;
use render;

use std::sync::Arc;
use std::sync::{Mutex,RwLock};
use std::collections::HashMap;

use super::Instance;

pub struct SceneAttrib {
    pub name:String,
}

pub struct Scene {
    pub attrib:RwLock<SceneAttrib>,
    pub instances:RwLock< HashMap<String,Arc<Instance>> >,
}

impl Scene {
    pub fn new(name:String, instances:HashMap<String,Arc<Instance>>) -> Self {
        Scene{
            attrib:RwLock::new(SceneAttrib{
                name:name,
            }),
            instances:RwLock::new( instances ),
        }
    }

    pub fn render(&self, frame:&mut render::Frame) -> Result<(),render::Error> {
        let instances_guard=self.instances.read().unwrap();

        for (_,instance) in instances_guard.iter() {
            instance.render(frame)?;
        }

        Ok(())
    }

    pub fn render_skeletons(&self, frame:&mut render::Frame) -> Result<(),render::Error> {
        let instances_guard=self.instances.read().unwrap();

        for (_,instance) in instances_guard.iter() {
            instance.render_skeleton(frame)?;
        }

        Ok(())
    }
}
