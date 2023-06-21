use bevy::prelude::*;
mod camera;
mod enemies;
mod plant_management;

use plant_management::*;
use enemies::*;
//
// let KUMARA: Plant = Plant { name: "Kumara", sow: Season::Autumn, harvest: Season::Spring,
// ability: Ability::Tank(10), last_used: Stopwatch::new(), delay: 2.0, };
//
// let MANUKA: Plant = Plant { name: "Manuka", sow: Season::Autumn, harvest: Season::Spring,
// ability: Ability::Shooter(10.), last_used: Stopwatch::new(), delay: 2.0, };
//
// let PUHA: Plant = Plant { name: "Puha", sow: Season::Autumn, harvest: Season::Spring,
// ability: Ability::AOE(10.), last_used: Stopwatch::new(), delay: 2.0, };

fn manage_attacks(
    mut commands: Commands,
    mut plant_query: Query<&PlantBed>,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
) 
{
    //let projectile = asset_server.load("projectile.png");

    for mut plant_bed in plant_query.iter_mut()
    {
        let mut plant = match &mut plant_bed.plant
        {
            Some(a) => a,
            None => continue,
        };

        if plant.last_used.elapsed_secs() < plant.delay { continue; }
        plant.last_used.reset();

        // if plant.ability == Ability::Shooter
        // {
        //     commands.spawn((
        //         SpriteBundle {
        //         texture: projectile.clone(),
        //         transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
        //         ..default()},
        //         Projectile { speed: 10., damage: 10 }));
        // }
        //
        // if plant.ability == Ability::Shooter
        // {
        //     println!("yuh");
        // }
    }

}

fn manage_enemies(mut query: Query<(&Enemy, &Health, &Transform)>,
                time: Res<Time>)
{
    // move all up
    for e in query.iter_mut()
    {
        let (mut enemy, mut health, mut trans) = e;

        // idk check if they hit then start attacking

        trans.translation.y -= enemy.speed * time.delta_seconds();
    }

    // check if they hit anything, if so stop and attack
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))

        .add_system(camera::camera_zoom)
        .add_system(plant_management::bed_interact)

        .add_startup_system(plant_management::spawn_beds)
        .add_startup_system(enemies::spawn_enemies)

        .add_system(manage_enemies)
        .add_system(manage_attacks)

        .add_system(manage_sticky)

        .run();
}
