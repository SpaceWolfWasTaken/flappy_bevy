use bevy::prelude::*;
mod player;
mod camera;
fn main() {
    App::new()
    .add_plugins(DefaultPlugins.set(WindowPlugin{
        primary_window: Some(Window {
            title:"Flappy Bevy".to_string(),
            resolution: (400., 700.).into(),
            resizable:false,
            ..default()
        }),
        ..default()
    }))
    .add_plugins(player::PlayerPlugin)
    .add_plugins(camera::CameraPlugin)
    .add_systems(Startup, setup)
    .run();
    
}

fn setup(
){    
    println!("Hello, world!");
}
