use aerin_ecs::Component;
use aerin_renderer::Shader;

pub struct ShaderComponent {
    pub shader: Option<Shader>,
}

impl Component for ShaderComponent {
    fn default() -> Self {
        Self { shader: None }
    }
    fn type_of() -> &'static str {
        "Shader"
    }
}
