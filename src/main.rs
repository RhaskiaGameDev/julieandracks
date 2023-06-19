use bevy::prelude::*;
mod gameinit;
mod camera;

fn main()
{
    App::new()
        .add_plugins(DefaultPlugins)
        .add_system(camera::camera_zoom)
        .add_startup_system(gameinit::spawn_beds)
        .run();
}