use bevy::prelude::*;
use bevy::ecs::component::Component;
use bevy::window::PrimaryWindow;

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
                PlantBed{ plant: None, row: x, column: y }));
        }
    }
}

// will hover over beds and interact with them
pub(crate) fn bed_interact(bed_query: Query<(&PlantBed, &Transform)>,
                           mouse_pos: Query<&Window, With<PrimaryWindow>>,
                           camera_q: Query<(&Camera, &GlobalTransform)>)
{
    // https://bevy-cheatbook.github.io/cookbook/cursor2world.html
    let (camera, camera_transform) = camera_q.single();

    if let Some(m_pos) = mouse_pos.single().cursor_position()
            .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
            .map(|ray| ray.origin.truncate())
    {
        for bed in bed_query.iter()
        {
            let bed_pos = Vec2::new(bed.1.translation.x, bed.1.translation.y);
            let rect = Rect::from_center_size(bed_pos, Vec2::ONE * 32.);

            if rect.contains(m_pos)
            {
                println!("hovered over row {}, and column {}", bed.0.row, bed.0.column);
            }
        }
    }
}