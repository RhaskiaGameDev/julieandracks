use bevy::prelude::*;
mod camera;
mod enemies;
mod plant_management;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_system(camera::camera_zoom)
        .add_system(plant_management::bed_interact)
        .add_startup_system(plant_management::spawn_beds)
        .add_startup_system(enemies::spawn_enemies)
        .run();
}
