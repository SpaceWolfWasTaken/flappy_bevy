use bevy::prelude::*;
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
    .add_systems(Startup, setup)
    .run();
    
}

fn setup(
    mut commands: Commands
){
    commands.spawn(Camera2dBundle::default());
    println!("Hello, world!");
}
