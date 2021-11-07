
use bevy::prelude::*;
use bevy_stimulus::*;

#[stimulus]
enum TestScheme {
    First
}

#[stimulus]
enum TestEvent {
    Bang,
    Boom
}

fn main() {
    let mut stimulus = Stimulus::new();
    stimulus.build_scheme(TestScheme::First, |s| {
        s.insert_action(TestEvent::Bang, Action::new(KeyCode::S, None));
    });

    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(StimulusPlugin::new(TestScheme::First, stimulus))
        .run();


}
