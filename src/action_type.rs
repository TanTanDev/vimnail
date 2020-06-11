use crate::direction::Direction;
use crate::item::ItemType;
use crate::mode::Mode;

#[derive(Clone, Copy)]
pub enum ActionType {
    ChangeMode(Mode),
    _PreviousMode,
    AddItem(ItemType),
    Move(Direction),
    Rotate(Direction),
    Scale(Direction),
    ScaleUniform(Direction),
}
