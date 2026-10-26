use aerin_ecs::Component;
use aerin_renderer::{Mesh, Shader, Texture};

pub struct QuadComponent {
    pub mesh: Option<Mesh>,
    pub shader: Option<Shader>,
    pub texture: Option<Texture>,
}

impl Component for QuadComponent {
    fn default() -> Self {
        Self {
            mesh: None,
            shader: None,
            texture: None,
        }
    }
    fn type_of() -> &'static str {
        "QuadComponent"
    }
}
