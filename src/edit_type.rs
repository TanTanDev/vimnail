#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum EditType {
    Move,
    Rotate,
    Scale,
    ScaleUniform,
}
