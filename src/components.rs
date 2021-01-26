use crate::prelude::*;

use std::collections::HashSet;

#[derive(Clone, Debug, PartialEq)]
pub struct FieldOfView {
    pub visible_tiles: HashSet<Point>,
    pub radius: i32,
    pub is_dirty: bool
}

impl FieldOfView {
    pub fn new(radius: i32)  -> Self {
        Self {
            visible_tiles: HashSet::new(),
            radius,
            // need to recalculate immediately
            is_dirty: true
        }
    }

    pub fn clone_dirty(&self) -> Self {
        Self {
            visible_tiles: HashSet::new(),
            radius: self.radius,
            is_dirty: true
        }
    }
}

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

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ChasingPlayer;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Render {
    pub color: ColorPair,
    pub glyph: FontCharType,
}

/// A message indicating that an entity wants to move somewhere
/// Components can be messages too, and we only have one system processing these to keep duplication
/// to a minimum
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WantsToMove {
    pub entity: Entity,
    pub destination: Point
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Health {
    pub current: i32,
    pub max: i32
}

#[derive(Clone, PartialEq)]
pub struct Name(pub String);

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WantsToAttack {
    pub attacker: Entity,
    pub victim: Entity
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ItemReceived {
    pub receiver: Entity,
    pub item: Entity
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Item;

// yet another lost amulet
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AmuletOfYala;

