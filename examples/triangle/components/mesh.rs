use aerin_ecs::Component;
use aerin_renderer::Mesh;

pub struct MeshComponent {
    pub mesh: Option<Mesh>,
}

impl Component for MeshComponent {
    fn default() -> Self {
        Self { mesh: None }
    }
    fn type_of() -> &'static str {
        "Mesh"
    }
}
