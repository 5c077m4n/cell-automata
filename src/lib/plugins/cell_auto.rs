use super::super::resources::decay_timer::DecayTimer;
use bevy::prelude::*;

const N_CUBES: usize = 10;
const CUBE_SIZE: f32 = 2.0;
const STATE_COUNT: usize = 5;

#[derive(Component)]
struct Cube;
#[derive(Component, Default, Debug)]
struct State {
	decay: bool,
	count: usize,
}

fn move_cubes(
	mut cube_query: Query<(&mut Transform, &GlobalTransform, &mut State), With<Cube>>,
	time: Res<Time>,
	mut timer: ResMut<DecayTimer>,
) {
	if timer.0.tick(time.delta()).just_finished() {
		let cubes: Vec<_> = cube_query.iter_mut().collect();

		let mut neighbour_count: usize = 0;
		for (trns, global_trns, _state) in &cubes {
			let cubes_clone: Vec<_> = cubes.iter().collect();

			let mut translation = trns.translation;
			let global_translation = global_trns.translation();

			for (curr_trns, curr_global_trns, _) in &cubes_clone {
				let curr_global_translation = curr_global_trns.translation();
				let (x, y, z) = (
					curr_trns.local_x(),
					curr_trns.local_y(),
					curr_trns.local_z(),
				);

				if curr_global_translation + x == global_translation {
					neighbour_count += 1;
				}
				if curr_global_translation - x == global_translation {
					neighbour_count += 1;
				}
				if curr_global_translation + y == global_translation {
					neighbour_count += 1;
				}
				if curr_global_translation - y == global_translation {
					neighbour_count += 1;
				}
				if curr_global_translation + z == global_translation {
					neighbour_count += 1;
				}
				if curr_global_translation - z == global_translation {
					neighbour_count += 1;
				}
				info!("{}", neighbour_count);
			}

			if neighbour_count == 4 {
				let direction = trns.local_x();
				translation += direction * time.delta_seconds();
			}
		}
	}
}
fn decay_step(
	mut commands: Commands,
	mut cube_query: Query<(Entity, &mut State, &ComputedVisibility), With<Cube>>,
	mut timer: ResMut<DecayTimer>,
	time: Res<Time>,
) {
	if timer.0.tick(time.delta()).just_finished() {
		for (entity, mut state, comp_vis) in cube_query.iter_mut() {
			if !comp_vis.is_visible() {
				commands.entity(entity).despawn();
			} else if state.decay {
				if let Some(n) = state.count.checked_sub(1) {
					state.count = n;
				}
				if state.count == 0 {
					commands.entity(entity).despawn();
				}
			}
		}
	}
}

fn setup(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
) {
	let half_n_cubes: isize = N_CUBES as isize / 2;
	for i in -half_n_cubes..half_n_cubes {
		commands
			.spawn_bundle(PbrBundle {
				mesh: meshes.add(Mesh::from(shape::Cube { size: CUBE_SIZE })),
				material: materials.add(Color::ANTIQUE_WHITE.into()),
				transform: Transform::from_xyz(
					(i % 10) as f32 * CUBE_SIZE,
					(i % 10) as f32 * CUBE_SIZE,
					0.0 * CUBE_SIZE,
				),
				..default()
			})
			.insert_bundle((
				Cube,
				State {
					decay: false,
					count: STATE_COUNT,
				},
			));
	}
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
