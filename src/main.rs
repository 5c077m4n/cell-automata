mod lib;

use bevy::{
	log::{Level, LogSettings},
	prelude::*,
};
use lib::plugins::cell_auto::CellAutomataPlugin;

fn main() {
	App::new()
		.insert_resource({
			if cfg!(debug_assertions) {
				LogSettings {
					filter: "info,wgpu_core=warn,wgpu_hal=warn,minewars=debug".into(),
					level: Level::DEBUG,
				}
			} else {
				LogSettings {
					filter: "warn".into(),
					level: Level::WARN,
				}
			}
		})
		.add_plugins(DefaultPlugins)
		.add_plugin(CellAutomataPlugin)
		.run();
}
