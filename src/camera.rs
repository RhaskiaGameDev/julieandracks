use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub fn camera_zoom(
    mut camera: Query<&mut OrthographicProjection>,
    window_query: Query<&Window, With<PrimaryWindow>>)
{
    let window = window_query.get_single().unwrap();

    camera.single_mut().scale = 480. / window.width();
}