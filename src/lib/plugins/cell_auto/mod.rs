mod components;
mod resources;
mod systems;

use bevy::prelude::*;
use components::{Cube, State};
use resources::DecayTimer;
use systems::{decay_step, move_cubes};

const N_CUBES: usize = 10;
const CUBE_SIZE: f32 = 2.0;
const STATE_COUNT: usize = 5;

fn setup(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
) {
	commands
		.spawn_bundle(PbrBundle {
			mesh: meshes.add(Mesh::from(shape::Plane { size: 10.0 })),
			material: materials.add(Color::NONE.into()),
			..default()
		})
		.with_children(|p| {
			let half_n_cubes: isize = N_CUBES as isize / 2;
			for i in -half_n_cubes..half_n_cubes {
				p.spawn_bundle(PbrBundle {
					mesh: meshes.add(Mesh::from(shape::Cube { size: CUBE_SIZE })),
					material: materials.add(Color::ANTIQUE_WHITE.into()),
					transform: Transform::from_xyz((i % 10) as f32 * CUBE_SIZE, 0.0, 0.0),
					..default()
				})
				.insert_bundle((Cube, State::new(STATE_COUNT)));
			}
		});
	commands.spawn_bundle(Camera3dBundle {
		transform: Transform::from_xyz(10.0, 10.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
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
		app.insert_resource(DecayTimer::default())
			.add_startup_system(setup)
			.add_system(move_cubes)
			.add_system(decay_step);
	}
}
