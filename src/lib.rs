use bevy::prelude::*;

mod stimulus;

pub use bevy_stimulus_derive::*;
pub use serde::{Deserialize, Serialize};
pub use stimulus::*;

pub struct StimulusPlugin<S, E> where S: Clone + Eq + std::hash::Hash, E: Clone + Eq + std::hash::Hash {
    default: S,
    stimulus: Stimulus<S, E>
}

impl<S, E> StimulusPlugin<S, E> where S: Clone + Eq + std::hash::Hash, E: Clone + Eq + std::hash::Hash {
    pub fn new(default: S, stimulus: Stimulus<S, E>) -> Self {
        Self {
            default,
            stimulus
        }
    }
}

impl<S: 'static + Send + Sync + Clone + Eq + std::hash::Hash + std::fmt::Display, E: 'static + Send + Sync + Clone + Eq + std::hash::Hash + std::fmt::Display> Plugin for StimulusPlugin<S, E> {
    fn build(&self, app: &mut AppBuilder) {
        app
            .insert_resource(self.default.clone())
            .insert_resource(self.stimulus.clone())
            .add_event::<E>()
            .add_system(stimulus_events::<S, E>.system());
    }
}

fn stimulus_events<S : Clone + Eq + std::hash::Hash + std::fmt::Display, E: 'static + Sync + Send + Clone + Eq + std::hash::Hash + std::fmt::Display>(mut events: EventWriter<E>, current: Res<S>, stimulus: Res<Stimulus<S, E>>, key_codes: Res<Input<KeyCode>>) {
    let scheme: Option<&Scheme<E>> = *stimulus.get_scheme(&*current);
    match scheme {
        None => {
            bevy::log::warn!("Current Scheme doesn't exist in Stimulus");
        }
        Some(scheme) => {

        }
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Formatter;
    use crate::*;

    #[stimulus]
    enum TestScheme {
        First
    }

    #[stimulus]
    enum TestEvent {
        Bang,
        Boom
    }

    #[test]
    fn serialize() {
        let mut stimulus: Stimulus<TestScheme, TestEvent> = Stimulus::new();
        stimulus.build_scheme(TestScheme::First, |s| {
            s
                .insert_action(TestEvent::Bang, Action::new(KeyCode::LShift, Some(2.0)))
                .insert_action(TestEvent::Boom, Action::new(KeyCode::S, None));
        });


        let result = toml::to_string(&stimulus).expect("Failed to serialize");
        println!("{}", result);
        assert_eq!(result, "[First.Boom]\ndigital = \"S\"\n\n[First.Bang]\ndigital = \"LShift\"\nscale = 2.0\n".to_owned());
    }
}
