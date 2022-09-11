use bevy::prelude::*;

#[derive(Component)]
pub struct Cube;
#[derive(Component, Default, Debug)]
pub struct State {
	pub decay: bool,
	pub count: usize,
}
impl State {
	pub fn new(count: usize) -> Self {
		Self {
			decay: false,
			count,
		}
	}
}
