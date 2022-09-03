use super::super::resources::state_timer::StateTimer;
use bevy::prelude::*;

#[derive(Component, Default, Debug)]
struct Cube;
#[derive(Component, Default, Debug)]
struct Location(usize, usize, usize);
#[derive(Component, Default, Debug)]
struct State(usize);

fn init_cubes(mut commands: Commands) {
	trace!("Creating cube objects");
	for i in 0..1000 {
		commands
			.spawn()
			.insert(Cube)
			.insert(Location(i % 10, i % 10, i % 10))
			.insert(State::default());
	}
}
fn move_cubes(
	mut cubes: Query<(&mut Transform, &mut Cube)>,
	mut timer: ResMut<StateTimer>,
	time: Res<Time>,
) {
	if timer.0.tick(time.delta()).just_finished() {
		for (mut transform, _cube) in &mut cubes {
			let forward = transform.forward();
			transform.translation += forward * time.delta_seconds();
		}
	}
}

fn draw(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
	cubes: Query<(&Location, &State), With<Cube>>,
) {
	for (location, _state) in cubes.iter() {
		let Location(x, y, z) = location;

		commands.spawn_bundle(PbrBundle {
			mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
			material: materials.add(Color::rgba(1.0, 1.0, 0.5, 0.75).into()),
			transform: Transform::from_xyz(*x as f32, *y as f32, *z as f32),
			..default()
		});
	}
	commands.spawn_bundle(Camera3dBundle {
		transform: Transform::from_xyz(0.0, 10.0, 20.0).looking_at(Vec3::ZERO, Vec3::Y),
		..default()
	});
	commands.spawn_bundle(PointLightBundle {
		transform: Transform::from_translation(Vec3::ONE * 3.0),
		..default()
	});
}

pub struct CellAutomataPlugin;
impl Plugin for CellAutomataPlugin {
	fn build(&self, app: &mut App) {
		app.insert_resource(StateTimer(Timer::from_seconds(1., true)))
			.add_startup_system(init_cubes)
			.add_startup_system(draw)
			.add_system(move_cubes);
	}
}
