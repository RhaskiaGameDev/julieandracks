use bevy::prelude::*;
use bevy::ecs::component::Component;

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
    pub plant: Option<Plant>,
    pub row: i32,
    pub column: i32
}