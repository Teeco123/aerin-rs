#[derive(Debug, Clone)]
pub struct WindowConfig {
    pub title: String,
    pub width: u32,
    pub height: u32,
    pub resizable: bool,
    pub visible: bool,
}
impl Default for WindowConfig {
    fn default() -> Self {
        Self {
            title: "Aerin Engine".to_string(),
            width: 1280,
            height: 720,
            resizable: true,
            visible: true,
        }
    }
}
