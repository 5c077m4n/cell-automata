use super::{
	components::{Cube, State},
	resources::DecayTimer,
};
use bevy::prelude::*;

pub fn decay_step(
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

pub fn move_cubes(
	mut cube_query: Query<(&mut Transform, &GlobalTransform, &mut State), With<Cube>>,
	time: Res<Time>,
	mut timer: ResMut<DecayTimer>,
) {
	if timer.0.tick(time.delta()).just_finished() {
		let cubes: Vec<_> = cube_query.iter_mut().collect();

		for (trns, global_trns, _state) in &cubes {
			let mut neighbour_count: usize = 0;
			let cubes_clone: Vec<_> = cubes.iter().collect();

			let mut translation = trns.translation;
			let global_translation = global_trns.translation();

			for (curr_trns, curr_global_trns, _) in &cubes_clone {
				let curr_global_translation = curr_global_trns.translation();

				if curr_global_translation != global_translation {
					let (x, y, z) = (
						curr_trns.local_x() * 2.,
						curr_trns.local_y() * 2.,
						curr_trns.local_z() * 2.,
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
				}
			}

			if neighbour_count > 1 {
				let direction = trns.local_y();
				translation += direction * time.delta_seconds();
			}
		}
	}
}
