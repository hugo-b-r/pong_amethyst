use bevy::prelude::*;

fn main() {
    App::new()
        .add_startup_system(add_people)
        .add_system(greet_people)
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
    commands.spawn((Person, Name("Elaina Proctor".to_string())));
    commands.spawn((Person, Name("Donald Duck".to_string())));
    commands.spawn((Person, Name("Mickey Mouse".to_string())));
}

fn greet_people(query: Query<&Name, With<Person>>) {
    for name in query.iter() {
        println!("Hello {}!", name.0);
    }
}