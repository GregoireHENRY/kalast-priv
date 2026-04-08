use crate::Mat4;

#[derive(Debug, Clone, Default)]
pub struct Body {
    pub mesh: Option<crate::mesh::Mesh>,
    pub mat: Mat4,
    pub entity: Option<crate::entity::Body>,
}