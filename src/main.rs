mod lib;

use bevy::{
	log::{Level, LogSettings},
	prelude::*,
	window::close_on_esc,
};
use lib::plugins::cell_auto::CellAutomataPlugin;

fn main() {
	App::new()
		.insert_resource({
			if cfg!(debug_assertions) {
				LogSettings {
					filter: "trace,debug,info,wgpu_core=warn,wgpu_hal=warn,minewars=debug".into(),
					level: Level::TRACE,
				}
			} else {
				LogSettings {
					filter: "warn".into(),
					level: Level::WARN,
				}
			}
		})
		.insert_resource(WindowDescriptor {
			title: "Cell Automata 3D".into(),
			..default()
		})
		.add_plugins(DefaultPlugins)
		.add_plugin(CellAutomataPlugin)
		.add_system(close_on_esc)
		.run();
}
