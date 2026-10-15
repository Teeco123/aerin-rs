#[derive(Debug, Clone, PartialEq)]
pub enum WindowEvent {
    Resized { width: u32, height: u32 },
    CloseRequested,
    Focused(bool),
    ScaleFactorChanged(f64),
    RedrawRequested,
}
