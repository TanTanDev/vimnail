use crate::mode::Mode;

#[derive(Clone, Copy)]
pub enum ActionType
{
    ChangeMode(Mode),
    PreviousMode,
}
