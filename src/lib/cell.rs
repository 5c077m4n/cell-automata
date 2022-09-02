#![allow(dead_code)]

use bevy::prelude::*;

#[derive(Component, Debug, Default)]
pub struct Cell {
	x: usize,
	y: usize,
	z: usize,
	color: (usize, usize, usize, usize),
}
