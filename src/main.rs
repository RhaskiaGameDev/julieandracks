use bevy::prelude::*;

#[derive(Component)]
struct Health
{
    health: i32,
}

fn main()
{
    App::new()
        .add_plugins(DefaultPlugins)
        .run();
}
