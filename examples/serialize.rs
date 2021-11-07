use bevy::prelude::*;
use bevy_stimulus::*;

#[stimulus]
enum MyProfile {
    Hello
}

#[stimulus]
enum MyEvent {
    Stomp,
    Push
}

fn main() {
    let mut stimulus: Stimulus<MyProfile, MyEvent> = Stimulus::new();
    stimulus.build_scheme(MyProfile::Hello, |p| {
        p
            .insert_action(MyEvent::Stomp, Action::new(KeyCode::LShift, Some(2.0)))
            .insert_action(MyEvent::Push, Action::new(KeyCode::S, None));
    });

    match toml::to_string(&stimulus) {
        Ok(s) => println!("{}", s),
        Err(e) => println!("{}", e)
    };
}