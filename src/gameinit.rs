use bevy::prelude::*;
use crate::gameinit::plants::PlantBed;
mod plants;

const ROWS: i32 = 4;
const COLUMNS: i32 = 5;

pub(crate) fn spawn_beds(mut commands: Commands,
                         asset_server: Res<AssetServer>)
{
    commands.spawn(Camera2dBundle::default());

    for x in 0..ROWS
    {
        for y in 0..COLUMNS
        {
            commands.spawn((
                SpriteBundle {
                    texture: asset_server.load("bed.png"),
                    ..default() },
                PlantBed{plant: None, row: x, column: y }));
        }
    }
}