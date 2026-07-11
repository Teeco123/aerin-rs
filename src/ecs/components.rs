pub type ComponentType = u8;

pub trait Component {
    fn default() -> Self;
    fn type_of() -> &'static str;
}
