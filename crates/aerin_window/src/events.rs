use aerin_input::KeyCode;

#[derive(Debug, Clone, PartialEq)]
pub enum WindowEvent {
    Resized { width: u32, height: u32 },
    CloseRequested,
    Focused(bool),
    ScaleFactorChanged(f64),
    RedrawRequested,

    KeyPressed(KeyCode),
    KeyReleased(KeyCode),
    MouseMoved { delta_x: f32, delta_y: f32 },
}
