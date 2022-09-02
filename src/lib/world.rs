use super::cell::Cell;
use bevy::prelude::*;

#[derive(Component, Default, Debug)]
pub struct World(pub Vec<Cell>);
