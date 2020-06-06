use std::hash::Hash;
use crate::insert_type::InsertType;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum Mode
{
    Command,
    Any,
    Insert,
    Edit,
    InsertType(InsertType),
}