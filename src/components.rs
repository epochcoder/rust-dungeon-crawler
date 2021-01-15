use crate::prelude::*;

// legion components are usually structs, but can also be enums
// all components will be in this file for now


/// Denotes a player component, not necessary to contain any fields, will act as a tag
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Player;

/// Denotes an enemy tagged component
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Enemy;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MovesRandomly;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Render {
    pub color: ColorPair,
    pub glyph: FontCharType,
}
