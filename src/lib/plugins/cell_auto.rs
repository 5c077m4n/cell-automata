use super::super::resources::state_timer::StateTimer;
use bevy::prelude::*;

#[derive(Component, Default, Debug)]
struct Cube;
#[derive(Component, Default, Debug)]
struct Location(usize, usize, usize);
#[derive(Component, Default, Debug)]
struct State(usize);

fn move_cubes(mut cubes: Query<&mut Transform, With<Cube>>, time: Res<Time>) {
	trace!("Moving cubes");
	for mut transform in &mut cubes {
		let direction = transform.local_x();
		transform.translation += direction * time.delta_seconds();
	}
}

fn setup(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
) {
	for i in -5..5 {
		commands
			.spawn_bundle(PbrBundle {
				mesh: meshes.add(Mesh::from(shape::Cube { size: 0.1 })),
				material: materials.add(Color::rgba(1.0, 1.0, 0.5, 0.75).into()),
				transform: Transform::from_xyz((i % 10) as f32, (i % 10) as f32, (i % 10) as f32),
				..default()
			})
			.insert(Cube)
			.insert(State(5));
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
			.add_startup_system(setup)
			.add_system(move_cubes);
	}
}
