use bevy::ecs::query::With;
use bevy::ecs::system::{Query,Res};
use bevy::math::Vec3;
use bevy::prelude::{Commands, Component, Plugin};
use bevy::time::Time;
use bevy::transform::components::Transform;
use bevy::app::{ Startup, Update,App};
use bevy::core_pipeline::core_2d::Camera2dBundle;

#[derive(Component)]
struct GameCamera;

pub struct CameraPlugin;

impl Plugin for CameraPlugin{
    fn build(&self, app: &mut App){
        app
        .add_systems(Startup, spawn_camera)
        .add_systems(Update,move_camera)
        ;
    }
}

fn spawn_camera(
    mut commands:Commands
){
    commands.spawn((
        Camera2dBundle::default(),
        GameCamera{}
    ));
}

fn move_camera(
    mut camera_query: Query<&mut Transform, With<GameCamera>>,
    time: Res<Time>
){
    let mut direction = Vec3::new(1.0,0.0,1.0);
    direction = direction.normalize_or_zero();
    if let Ok(mut transform) = camera_query.get_single_mut() {
        transform.translation += direction * 100.0 * time.delta_seconds();     
    }
}