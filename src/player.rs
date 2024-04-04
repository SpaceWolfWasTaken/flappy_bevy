use bevy::ecs::query::With;
use bevy::ecs::system::{Query,Res,ResMut};
use bevy::input::mouse::MouseButton;
use bevy::input::ButtonInput;
use bevy::math::primitives::Circle;
use bevy::math::Vec3;
use bevy::prelude::{default, App, Assets, ColorMaterial, Commands, Component, Mesh, Plugin};
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use bevy::render::color::Color;
use bevy::time::Time;
use bevy::transform::components::Transform;
use bevy::app::{ Startup, Update};
pub struct PlayerPlugin;

pub const PLAYER_SIZE:f32 = 25.0;

#[derive(Component)]
struct Player{}

#[derive(Component)]
struct Gravity{}

impl Plugin for PlayerPlugin{
    fn build(&self, app: &mut App){
        app
        .add_systems(Startup, spawn_player)
        .add_systems(Update,move_player)
        .add_systems(Update, gravity)
        ;
    }
}

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>
){
    let circle = Mesh2dHandle(meshes.add(Circle{radius:PLAYER_SIZE}));
    let color = Color::rgb(0.9,0.85,0.1);

    commands.spawn((
        MaterialMesh2dBundle{
            mesh:circle,
            material:materials.add(color),
            transform:Transform::from_xyz(0.0,0.0,1.),
            ..default()
        },
        Player{},
        Gravity{}
    ));
}

fn move_player(
    buttons:Res<ButtonInput<MouseButton>>,
    mut player_query:Query<&mut Transform, With<Player>>,
    time: Res<Time>
){
    if let Ok(mut transform) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;
        if buttons.just_pressed(MouseButton::Left){
            println!("Left button was pressed!");
            direction += Vec3::new(1.0,1.0,1.0); //top-right
        }
        direction = direction.normalize_or_zero();

        transform.translation += direction * 600.0 * time.delta_seconds();
    }
}

fn gravity(
    mut gravity_query:Query<&mut Transform, With<Gravity>>,
    time: Res<Time>
){
    let mut direction = Vec3::new(0.0,-1.0,1.0);
    direction = direction.normalize_or_zero();
    for mut entities in gravity_query.iter_mut(){
        entities.translation += direction * 25.0 * time.delta_seconds();     
    }
}