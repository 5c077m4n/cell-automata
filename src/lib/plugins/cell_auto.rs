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

fn scene(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
) {
	// cube
	commands.spawn_bundle(PbrBundle {
		mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
		material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
		transform: Transform::from_xyz(0.0, 0.5, 0.0),
		..default()
	});
	// light
	commands.spawn_bundle(PointLightBundle {
		point_light: PointLight {
			intensity: 1500.0,
			shadows_enabled: true,
			..default()
		},
		transform: Transform::from_xyz(4.0, 8.0, 4.0),
		..default()
	});
	// camera
	commands.spawn_bundle(Camera3dBundle {
		transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
		..default()
	});
}

pub struct CellAutomataPlugin;
impl Plugin for CellAutomataPlugin {
	fn build(&self, app: &mut App) {
		app.insert_resource(StateTimer(Timer::from_seconds(1., true)))
			.add_startup_system(add_cells)
			.add_startup_system(scene)
			.add_system(debug_cube);
	}
}
