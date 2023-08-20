use bevy::prelude::*;

// --- Hello World
// fn main() {
//     App::new().add_systems(Startup, hello_world).run();
// }

// pub fn hello_world() {
//     println!("hello world");
// }

// --- App
pub fn basics_app() {
    App::new()
        .add_plugins(( DefaultPlugins, PeoplePlugin ))
        .add_systems(Startup, setup)
        .run();
        // - Instead of adding systems here, we can add them in the plugin
        //   via `PeoplePlugin` above
        // .add_systems(
        //     Update,
        //     // (print_names, people_with_jobs, people_ready_for_hire),
        //     person_does_job,
        // )
}

// --- Plugin
struct PeoplePlugin;

impl Plugin for PeoplePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (print_names, people_with_jobs, people_ready_for_hire),
        );
    }
}

// --- Systems
fn setup(mut commands: Commands) {
    commands.spawn((
        Person {
            name: "Hiro".to_string(),
        },
        Employed {
            job: Job::Programmer,
        },
    ));
    commands.spawn(Person {
        name: "Yuri".to_string(),
    });
    commands.spawn((
        Person {
            name: "Yoko".to_string(),
        },
        Employed { job: Job::Artist },
    ));
    commands.spawn((
        Person {
            name: "Akari".to_string(),
        },
        Employed { job: Job::Musician },
    ));
}

fn print_names(person_query: Query<&Person>) {
    for person in person_query.iter() {
        println!("Name: {}", person.name);
    }
}

fn people_with_jobs(person_query: Query<&Person, With<Employed>>) {
    for person in person_query.iter() {
        println!("Employed Name: {}", person.name);
    }
}

fn people_ready_for_hire(person_query: Query<&Person, Without<Employed>>) {
    for person in person_query.iter() {
        println!("Unemployed Name: {}", person.name);
    }
}

fn person_does_job(person_query: Query<(&Person, &Employed)>) {
    for (person, employed) in person_query.iter() {
        println!("{} does {:?}", person.name, employed.job);
    }
}

// --- Components
#[derive(Component)]
struct Person {
    pub name: String,
}

#[derive(Debug)]
enum Job {
    Programmer,
    Artist,
    Musician,
}

#[derive(Component)]
struct Employed {
    pub job: Job,
}
