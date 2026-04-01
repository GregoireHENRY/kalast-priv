use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
pub struct Body {
    pub mesh: Option<Rc<RefCell<crate::mesh::Mesh>>>,
    pub instance: super::gpu::InstanceInput,
    pub entity: Option<crate::entity::Body>,
}

impl Body {
    pub fn new() -> Self {
        Self {
            mesh: None,
            instance: super::gpu::InstanceInput::default(),
            entity: None,
        }
    }
}
