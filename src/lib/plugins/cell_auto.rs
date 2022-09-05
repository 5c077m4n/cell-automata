use super::super::resources::decay_timer::DecayTimer;
use bevy::prelude::*;

const N_CUBES: usize = 10;
const CUBE_SIZE: f32 = 2.0;

#[derive(Component)]
struct Cube;
#[derive(Component, Default, Debug)]
struct State {
	decay: bool,
	count: usize,
}

fn move_cubes(mut cube_query: Query<&mut Transform, With<Cube>>, time: Res<Time>) {
	for mut transform in cube_query.iter_mut() {
		let direction = transform.local_x();
		transform.translation += direction * time.delta_seconds();
	}
}
fn decay_step(
	mut commands: Commands,
	mut cube_query: Query<
		(
			Entity,
			&mut State,
			&mut ComputedVisibility,
			Option<&mut UiColor>,
		),
		With<Cube>,
	>,
	mut timer: ResMut<DecayTimer>,
	time: Res<Time>,
) {
	if timer.0.tick(time.delta()).just_finished() {
		for (entity, mut state, comp_vis, ui_color) in cube_query.iter_mut() {
			if !comp_vis.is_visible() {
				commands.entity(entity).despawn();
			} else if state.decay {
				if let Some(n) = state.count.checked_sub(1) {
					state.count = n;
				}
				if state.count == 0 {
					commands.entity(entity).despawn();
				} else if let Some(mut ui_color) = ui_color {
					ui_color.0.set_a(1.0 - (1.0 - state.count as f32));
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
				material: materials.add(Color::rgba(1.0, 1.0, 0.5, 1.0).into()),
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
					decay: i % 2 == 0,
					count: 5,
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
