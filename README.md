# Bevy Game Engine "Getting Started"

Setting up as easy as creating new cargo project, adding Bevy as dependancy.

# Compile with Performance Optimisation

```toml
# Enable a small amount of optimization in the dev profile.
[profile.dev]
opt-level = 1

# Enable a large amount of optimization in the dev profile for dependencies.
[profile.dev.package."*"]
opt-level = 3
```

# Apps

Every Bevy program is an `App`. Simplest app can be

```rust
use bevy::prelude::*;

fn main() {
    App::new().run();
}
```

This app does not do anything. Apps are empty shells capable of running application logic. Apps hold three fields: `world` , `schedule` and `runner`. The world fields stores game data and schedule holds systems that operate on this data together the order they do so. Runner interprets the schedule to control the broad execution strategy.

We can use `bilder pattern` on App that lets us do things like initialising resourses in the world to store globally available data that we only need a single copy of, adding systems to schedule, which can read and modify resources and our entities' components according to game logic, as well as it can import other blocks of App-modifying code using Plugins.

# ECS

All app logic in Bevy uses the Entity Component System. Entities are unique "things" that are assigned groups of Components, which are then processed using Systems.

E.g an entity can have a `Position` and `Velocity` component, whereas another entity can have a `Position` and `UI` component. Systems are logic that runs on a specific set of component types. We might have a `movement` system that runs on all entities with a `Position` and `Velocity` component.

ECS ecnourages clean, decoupled designs by forcing you to break up yourr app data and logic into its core components. It also makes code faster by optimising memory access patterns and making parallelism easier.

# Bevy ECS

Unlike other Rust ECS implementations, which often require complex lifetimes, traits, builder patterns or macros, Bevy ECS uses normal Rust datatypes for all of these concepts:

- **Components**: Rust structs that implement the [`Component`](https://docs.rs/bevy/latest/bevy/ecs/component/trait.Component.html) trait

```rust
#[derive(Component)]
struct Position {
    x: f32,
    y: f32,
}
```

- **Systems**: normal Rust functions

```rust
fn print_position_system(query: Query<&Position>) {
    for position in &query {
        println!("position: {} {}", position.x, position.y);
    }
}
```

- **Entities**: a simple type containing a unique integer

```rust
struct Entity(u64);
```

# Simple system

Creating a system, using Update schedule and running it. Resulting in a program printing hello world.

```rust
use bevy::app::{App, Update};

fn main() {
    App::new().add_systems(Update, hello_world).run();
}

fn hello_world() {
    println!("Hello world!");
}
```

# Components

Let's greet a person instead of the whole world. In ECS, you would generally model people as entities with a set of components that define them.

We also want people to have names. Normally, we might just add a field to a Person struct called 'name'. But other entities might have names too. For example, dogs should probably have a name. It often makes sense to break datatypes up in to small pieces to encourage code reuse.

```rust
#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);
```

We can add some people to our World using "startup systems". Startup systems are just like normal systems, but they run just once, before all other systems, right when our App starts. We can use Commands to spawn entities into our World

```rust
fn main() {
    App::new()
        .add_systems(Startup, add_people)
        .add_systems(Update, hello_world)
        .run();
}

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Ziomek".to_string())));
    commands.spawn((Person, Name("Mateusz".to_string())));
    commands.spawn((Person, Name("Adam".to_string())));
}
```

Commands can be used to e.g. spawn or despawn entities, insert components on new or existing entities, inserting resources.

# Query

Running the above example does not do much. Here, we actually iterate over people we have added and print their greet each of them.

```rust
fn main() {
    App::new()
        .add_systems(Startup, add_people)
        .add_systems(Update, (hello_world, greet_people))
        .run();
}

fn greet_people(query: Query<&Name, With<Person>>) {
    for name in &query {
        println!("Hello {}!", name.0);
    }
}
```

Parameters passed into a "system function" define what data the system runs on. In this case, `greet_people` will run on all entities with `Person` and `Name` component. We can look at Query above as "iterate over every `Name` component for entities that also have a `Person` component."

`With` makes it so we query only entities having "Person" component, but don't actually care about the value of it.

## Mutable query

```rust
fn main() {
    App::new()
        .add_systems(Startup, add_people)
        .add_systems(Update, (hello_world, (update_people, greet_people).chain()))
        .run();
}
fn update_people(mut query: Query<&mut Name, With<Person>>) {
    for mut name in &mut query {
        if name.0 == "Ziomek" {
            name.0 = "Ziomek Ross".to_string();
            break;
        }
    }
}
```

To mutate properties of an entity, we can create use a mutable query. We are using `chain()` method on the two systems, which ensures they run in the specified order, so that the name gets updated before printing hello.

# Plugins

Bevy values modularity. All Bevy engine features are implemented as plugins - collections of code that modify an App. Internal features like the renderer, but games themselves can be implemented as Plugins.

Adding a third party plugin is as easy as installing a dependency, importing the plugin and using `app.add_plugins(plugin)`.

Most developers don't need custom experience and want "full engine" experience, for this Bevy provides a set of `DefaultPlugins`.

Adding default plugins and running our program will spawn a window and add an event loop which prints our output infinitely. Our App's Schedule now runs in a loop once per "frame".

We can also do that with our code and make it a Plugin instead:

```rust
pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, add_people);
        app.add_systems(Update, (hello_world, (update_people, greet_people).chain()));
    }
}
```

# Resources

Entity and Component data types are great for representing comples, query-able groups of data. But most apps require "globally unique" data of some kind. These kinds of data implement the `Resource` trait.

Some examples of data that could be a Resource:

- Elapsed time
- Asset Collection (sounds, textures, meshes)
- Renderers

# Tracking time with resources

To fix the spamming issue from our example, we can implement the `Time` resource, which is automatically added with the Default Plugins.

```rust
#[derive(Resource)]
struct GreetTimer(Timer);

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)));
        app.add_systems(Startup, add_people);
        app.add_systems(Update, (update_people, greet_people).chain());
    }
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
```

`Res` and `ResMut` pointers provide read and write access (respectively) to resources. The delta field on Time gives us the time that has passed since the last update. We use our own GreetTimer resource to keep track of the amount of time that has passed over a series of updates. We are utilising Bevy's `Timer` type for this.
