use super::super::resources::state_timer::StateTimer;
use bevy::prelude::*;

#[allow(dead_code)]
#[derive(Component, Debug, Default)]
struct Cell {
	location: (usize, usize, usize),
	color: (usize, usize, usize, usize),
	state: usize,
}

#[allow(dead_code)]
#[derive(Component, Default, Debug)]
struct Cube {
	cells: Vec<Cell>,
	size: usize,
}
impl Cube {
	pub fn add_cell(&mut self, cell: Cell) {
		self.cells.push(cell);
	}
}

fn add_cells(mut commands: Commands) {
	let mut cube = Cube::default();
	for _ in 0..100 {
		cube.add_cell(Cell::default());
	}
	commands.spawn().insert(cube);
}
fn debug_cube(time: Res<Time>, mut timer: ResMut<StateTimer>, query: Query<&Cube>) {
	if timer.0.tick(time.delta()).just_finished() {
		let world = query.single();
		println!("{:#?}", &world);
	}
}

pub struct CellAutomataPlugin;
impl Plugin for CellAutomataPlugin {
	fn build(&self, app: &mut App) {
		app.insert_resource(StateTimer(Timer::from_seconds(1., true)))
			.add_startup_system(add_cells)
			.add_system(debug_cube);
	}
}
