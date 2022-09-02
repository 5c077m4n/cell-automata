mod lib;

use bevy::prelude::*;
use lib::plugins::cell_auto::CellAutomataPlugin;

fn main() {
	App::new()
		.add_plugins(DefaultPlugins)
		.add_plugin(CellAutomataPlugin)
		.run();
}
