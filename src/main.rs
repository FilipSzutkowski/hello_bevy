use bevy::{
    app::{App, Plugin, Startup, Update},
    prelude::{Commands, Component, IntoSystemConfigs, Query, Res, ResMut, Resource, With},
    time::{Time, Timer, TimerMode},
    DefaultPlugins,
};

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

#[derive(Resource)]
struct GreetTimer(Timer);
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(HelloPlugin)
        .run();
}

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)));
        app.add_systems(Startup, add_people);
        app.add_systems(Update, (update_people, greet_people).chain());
    }
}

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Ziomek".to_string())));
    commands.spawn((Person, Name("Mateusz".to_string())));
    commands.spawn((Person, Name("Adam".to_string())));
}

fn greet_people(
    time: Res<Time>,
    mut greet_timer: ResMut<GreetTimer>,
    query: Query<&Name, With<Person>>,
) {
    // Update timer with the time elapsed since last update.
    // if that caused the timer to finish, we say hello
    if greet_timer.0.tick(time.delta()).just_finished() {
        for name in &query {
            println!("Hello {}!", name.0);
        }
    }
}

fn update_people(mut query: Query<&mut Name, With<Person>>) {
    for mut name in &mut query {
        if name.0 == "Ziomek" {
            name.0 = "Ziomek Ross".to_string();
            break;
        }
    }
}
