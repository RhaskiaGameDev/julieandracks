use bevy::prelude::*;
mod camera;
mod enemies;
mod plant_management;

use plant_management::*;
use enemies::*;

fn manage_attacks(
    commands: Commands,
    plant_query: Query<PlantBed>,
    time: Res<Time>,
) 
{
    for plant: PlantBed in plant_query.iter_mut()
    {
        if plant.plant.last_used.elapsed_secs < plant.plant.delay { continue; }
        plant.last_used.reset();

        match plant.plant.ability 
        {
            Shooter(p) => commands.spawn((SpriteBundle {
                texture: bed_sprite.clone(),
                transform: Transform::from_translation(Vec3::new(32. * (x as f32 - 2.), 32. * (y as f32 - 3.), 0.)),
                ..default()}),
                Projectile { speed: 10., pow: 10 }),
            AOE(r, s) => 
        }
    }

}

fn manage_enemies()
{
    // move all up

    // check if they hit anything, if so stop and attack
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_system(camera::camera_zoom)
        .add_system(plant_management::bed_interact)
        .add_startup_system(plant_management::spawn_beds)
        .add_startup_system(enemies::spawn_enemies)
        .run();
}
