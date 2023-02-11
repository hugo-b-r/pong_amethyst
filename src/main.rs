use bevy::prelude::*;

fn main() {
    App::new()
        .add_startup_system(add_people)
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

fn add_people(mut commands: Commands) {
    commmands.spawn((Person, Name("Elaina Proctor".to_string())));
    commmands.spawn((Person, Name("Donald Duck".to_string())));
    commmands.spawn((Person, Name("Mickey Mouse".to_string())));
}