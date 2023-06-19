use bevy::prelude::*;
mod gameinit;

fn main()
{
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(gameinit::spawn_beds)
        .run();
}