use bevy::prelude::*;

fn main() {
    App::new().add_startup_system(setup_camera)
                .add_plugins(DefaultPlugins)
                .run();
        
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

#[derive(Component)]
struct SnakeHead;

const SNAKE_HEAD_COLOR: Color = Color::rgb(0.7, 0.7, 0.7);
