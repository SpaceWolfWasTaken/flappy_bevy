use bevy::ecs::query::With;
use bevy::ecs::system::{Query,Res,ResMut};
use bevy::input::{ButtonInput,mouse::MouseButton};
use bevy::prelude::{default, App, Assets, ColorMaterial, Commands, Component, Mesh, Plugin};
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use bevy::render::color::Color;
use bevy::transform::components::Transform;
use bevy::app::{Update,Startup};
use bevy::window::{Window,PrimaryWindow};
use bevy::math::{primitives::Rectangle,Vec2};
use super::camera;
pub struct PipePlugin;

#[derive(Component)]
struct Pipe{}

impl Plugin for PipePlugin{
    fn build(&self, app: &mut App){
        app
        .add_systems(Startup, spawn_pipe_static)
        .add_systems(Update, spawn_pipe_static)
        ;
    }
}

pub fn spawn_pipe_static(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut meshes:ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    buttons:Res<ButtonInput<MouseButton>>,
    mut camera_query:Query<&Transform, With<camera::GameCamera>>
){
    //WRITE CODE THAT SPAWNS A STATIC PIPE EVERY 100 CAMERA POS.
    if buttons.just_pressed(MouseButton::Left){
    println!("Spawning Pipe!");
    let window_height = window_query.get_single().unwrap().height();
    let window_width = window_query.get_single().unwrap().width();
    let height = 0.5 * window_height; //length of rect. area is 2l. so 0.5 * height makes it take the entire window height
    let width:f32 = 50.0;
    let rect = Mesh2dHandle(meshes.add(Rectangle{half_size:Vec2::new(width,height)}));
    let color = Color::rgb(0.24, 0.8, 0.24);

    let camera = camera_query.get_single_mut().unwrap();
    let y = 0.0; //middle of screen (top-bot) //needs to change when dynamically generating pipes in y-axis
    let x =camera.translation.x + window_width/1.6; //perfect when width is 50
    let transform = Transform::from_xyz(x, y, 1.);
    commands.spawn((MaterialMesh2dBundle{
        mesh:rect,
        material:materials.add(color),
        transform: transform,
        ..default()
        },Pipe{}));
    }
}