pub trait Component {
    fn default() -> Self;
    fn type_of() -> &'static str;
}
