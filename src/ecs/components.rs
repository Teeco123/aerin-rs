pub type ComponentType = u8;

pub trait Component {
    fn default() -> Self;
    fn type_of() -> &'static str;
}

pub struct Position {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Component for Position {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
    fn type_of() -> &'static str {
        "Position"
    }
}
