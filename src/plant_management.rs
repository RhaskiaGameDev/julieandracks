use bevy::prelude::*;
use bevy::a11y::{accesskit::{NodeBuilder, Role}, AccessibilityNode};
use bevy::input::mouse::{MouseScrollUnit, MouseWheel};
use bevy::winit::WinitSettings;
use bevy::ecs::component::Component;
use bevy::window::PrimaryWindow;
use bevy::time::Stopwatch;

#[derive(Component)]
pub struct Health
{
    health: i32,
}

pub enum StickyEnum
{
    Left (f32),
    Right (f32),
    Up (f32),
    Down (f32),
}

#[derive(Component)]
pub struct Sticky
{
    dir: StickyEnum,
}

pub fn manage_sticky(mut sticky_query: Query<(&Sticky, &mut Transform)>,
                     window_query: Query<&Window, With<PrimaryWindow>>)
{
    let window = window_query.get_single().unwrap();
    let width = window.width();
    let height = window.height();

     for mut sticky in sticky_query.iter_mut()
     {
         let mut trans: &mut Transform = &mut sticky.1;
         let sticky: &Sticky = sticky.0;
         match sticky.dir
         {
             StickyEnum::Left(d) => trans.translation.x = -width / 2. + d,
             StickyEnum::Right(d) => trans.translation.x = width / 2. + d,
             StickyEnum::Up(d) => trans.translation.y = -height / 2. + d,
             StickyEnum::Down(d) => trans.translation.y = height / 2. + d,
         }
     }
}

#[derive(Clone, PartialEq, Copy, Debug)]
pub enum Season
{
    Spring,
    Summer,
    Autumn,
    Winter,
}

#[derive(Clone, PartialEq, Copy, Debug)]
pub enum Ability 
{
    Shooter (f32), // attack
    AOE (f32),
    Tank (i32),
    None,
}

#[derive(Component, Clone, PartialEq, Debug)]
pub struct Projectile 
{
    pub(crate) speed: f32,
    pub(crate) damage: i32,
}

#[derive(Clone, Copy, Debug)]
pub struct Plant
{
    name: &'static str,
    sow: Season,
    harvest: Season,
    pub(crate) ability: Ability,
    pub(crate) last_used: f32,
    pub(crate) delay: f32,
}

#[derive(Component, Debug)]
pub struct PlantBed
{
    pub plant: Option<Plant>,
    pub row: i32,
    pub column: i32
}

#[derive(Component)]
pub struct SeedBag
{
    pub seeds: Vec<Plant>,
    pub selected: usize,
}

const ROWS: i32 = 5;
const COLUMNS: i32 = 4;

pub(crate) fn spawn_beds(mut commands: Commands,
                         asset_server: Res<AssetServer>)
{
    commands.spawn(Camera2dBundle::default());

    commands.spawn(SeedBag { seeds: vec![
        Plant { name: "Kumara", sow: Season::Spring, harvest: Season::Autumn, ability: Ability::None, last_used: 0.0, delay: 2.0 } ],
        selected:0});


    commands.spawn(
        SpriteBundle {
        texture: asset_server.load("shelfleft.png"),
        transform: Transform::from_translation(Vec3::new(170., 80., 0.)),
        ..default() });

    commands.spawn(
        SpriteBundle {
            texture: asset_server.load("shelfleft.png"),
            transform: Transform::from_translation(Vec3::new(-170., 80., 0.)),
            ..default() });


    let bed_sprite =  asset_server.load("bed.png");

    for x in 0..ROWS
    {
        for y in 0..COLUMNS
        {
            commands.spawn((
                SpriteBundle {
                    texture: bed_sprite.clone(),
                    transform: Transform::from_translation(Vec3::new(32. * (x as f32 - 2.), 32. * (y as f32 + 0.5), (5-y) as f32)),
                    ..default() },
                PlantBed{ plant: None, row: x, column: y },
                Health { health: 100 }));
        }
    }
}

//pub(crate) spawn_UI
//{

//}





pub(crate) fn show_seeds(seed_bag: Query<&SeedBag>,
                        )
{
    // ui stuff ig
}