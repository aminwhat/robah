use bevy::{prelude::*, winit::WinitSettings};
use dev_tools::DevToolsPlugin;
use prelude::PreludePlugin;

mod features;
mod prelude;

fn main() {
    let mut app = App::new();

    app
        // Only run the app when there is user input. This will significantly reduce CPU/GPU use.
        .insert_resource(WinitSettings::desktop_app())
        .add_plugins(DevToolsPlugin)
        .add_plugins(PreludePlugin);

    app.run();
}
