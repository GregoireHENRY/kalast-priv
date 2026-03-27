use std::{cell::RefCell, rc::Rc};

use glam::Mat4;

#[derive(Debug)]
pub struct Body {
    pub mesh: Option<Rc<RefCell<crate::mesh::Mesh>>>,
    pub mat: crate::Mat4,
    pub entity: Option<crate::entity::Body>,
}

impl Body {
    pub fn new() -> Self {
        Self {
            mesh: None,
            mat: Mat4::IDENTITY,
            entity: None,
        }
    }
}
