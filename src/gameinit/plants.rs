use bevy::prelude::*;

#[derive(Component)]
pub struct Health
{
    health: i32,
}

pub struct Plant
{
    name: String,
}

#[derive(Component)]
pub struct PlantBed
{
    plant: Option<Plant>,
    row: i32,
    column: i32
}