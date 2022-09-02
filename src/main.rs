mod lib;

use bevy::prelude::*;
use lib::{cell, world};

fn add_cells(mut commands: Commands) {
	let mut world = world::World::default();
	for _ in 0..100 {
		world.0.push(cell::Cell::default());
	}
	commands.spawn().insert(world);
}

fn log_worlds(query: Query<&world::World>) {
	let world = query.single();
	println!("{:#?}", &world);
}

fn main() {
	App::new()
		.add_plugins(DefaultPlugins)
		.add_startup_system(add_cells)
		.add_system(log_worlds)
		.run();
}
