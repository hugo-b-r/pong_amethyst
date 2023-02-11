use bevy::prelude::*;

fn main() {
    App::new()
        .add_system(hello_world)
        .run();
}

fn hello_world() {
    println!("hello world!");
}

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);