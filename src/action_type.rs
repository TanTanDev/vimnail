use crate::mode::Mode;
use crate::item::{ItemType};
use crate::direction::Direction;

#[derive(Clone, Copy)]
pub enum ActionType
{
    ChangeMode(Mode),
    PreviousMode,
    AddItem(ItemType),
    Move(Direction),
    Rotate(Direction),
    Scale(Direction),
    ScaleUniform(Direction),
}
