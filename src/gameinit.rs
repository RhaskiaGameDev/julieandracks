use bevy::prelude::*;
use crate::gameinit::plants::PlantBed;
mod plants;

const ROWS: i32 = 5;
const COLUMNS: i32 = 4;

pub(crate) fn spawn_beds(mut commands: Commands,
                         asset_server: Res<AssetServer>)
{
    commands.spawn(Camera2dBundle::default());

    let bed_sprite =  asset_server.load("bed.png");

    for x in 0..ROWS
    {
        for y in 0..COLUMNS
        {
            commands.spawn((
                SpriteBundle {

                    texture: bed_sprite.clone(),
                    transform: Transform::from_translation(Vec3::new(32. * (x as f32 - 2.), 32. * (y as f32 - 1.5), 0.)),
                    ..default() },
                PlantBed{plant: None, row: x, column: y }));
        }
    }
}