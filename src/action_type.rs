use crate::direction::Direction;
use crate::item::ItemType;
use crate::mode::Mode;
use crate::command_type::CommandType;

#[derive(Clone, Copy)]
pub enum ActionType {
    ChangeMode(Mode),
    _PreviousMode,
    AddItem(ItemType),
    Move(Direction),
    Rotate(Direction),
    Scale(Direction),
    ScaleUniform(Direction),
    CommandType(CommandType),
    RunCommand,
}
