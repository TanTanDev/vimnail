use crate::edit_type::EditType;
use crate::insert_type::InsertType;
use crate::item::ItemType;
use std::hash::Hash;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum Mode {
    Command,
    Any,
    Insert,
    Edit,
    EditType(EditType),
    InsertType(ItemType),
}

impl std::fmt::Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{:?}", self)
    }
}
