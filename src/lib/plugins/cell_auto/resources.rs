use bevy::prelude::*;

pub struct DecayTimer(pub Timer);

impl Default for DecayTimer {
	fn default() -> Self {
		Self(Timer::from_seconds(1., true))
	}
}
