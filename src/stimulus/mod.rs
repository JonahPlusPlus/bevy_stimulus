use bevy::utils::HashMap;
use std::fmt::Formatter;
use serde::*;

mod action;
pub use action::*;

mod scheme;
pub use scheme::*;

mod stimuli;
pub use stimuli::*;

#[derive(Clone)]
pub struct Stimulus<S: Clone + Eq + std::hash::Hash, E: Clone + Eq + std::hash::Hash> {
    schemes: HashMap<S, Scheme<E>>
}

#[cfg(feature = "serialize")]
impl<S: Serialize + Clone + Eq + std::hash::Hash + std::fmt::Display, E: Serialize + Clone + Eq + std::hash::Hash + std::fmt::Display> Serialize for Stimulus<S, E> {
    fn serialize<T: Serializer>(&self, serializer: T) -> Result<T::Ok, T::Error>
    {
        use ser::SerializeMap;
        let mut map = serializer.serialize_map(Some(self.schemes.len()))?;
        for (k, v) in &self.schemes {
            map.serialize_entry(&k.to_string(), &v)?;
        }
        map.end()
    }
}

struct StimulusVisitor<S: Clone + Eq + std::hash::Hash + std::fmt::Display, E: Clone + Eq + std::hash::Hash + std::fmt::Display> {
    marker: std::marker::PhantomData<fn() -> Stimulus<S, E>>
}

impl<S: Clone + Eq + std::hash::Hash + std::fmt::Display, E: Clone + Eq + std::hash::Hash + std::fmt::Display> StimulusVisitor<S, E> {
    fn new() -> Self {
        StimulusVisitor {
            marker: std::marker::PhantomData
        }
    }
}

impl<'de, S: Deserialize<'de> + Clone + Eq + std::hash::Hash + std::fmt::Display, E: Deserialize<'de> + Clone + Eq + std::hash::Hash + std::fmt::Display> serde::de::Visitor<'de> for StimulusVisitor<S, E> {
    type Value = Stimulus<S, E>;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("Scheme HashMap")
    }

    fn visit_map<M: serde::de::MapAccess<'de>>(self, mut access: M) -> Result<Self::Value, M::Error> {
        let mut schemes = HashMap::default();
        while let Some((k, v)) = access.next_entry()? {
            schemes.insert(k, v);
        }
        let stimulus = Stimulus {
            schemes
        };
        Ok(stimulus)
    }
}


#[cfg(feature = "serialize")]
impl<'de, S: Deserialize<'de> + Clone + Eq + std::hash::Hash + std::fmt::Display, E: Deserialize<'de> + Clone + Eq + std::hash::Hash + std::fmt::Display> Deserialize<'de> for Stimulus<S, E> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_map(StimulusVisitor::new())
    }
}

impl<S: Clone + Eq + std::hash::Hash + std::fmt::Display, E: Clone + Eq + std::hash::Hash + std::fmt::Display> Stimulus<S, E> {
    pub fn new() -> Self {
        Self {
            schemes: Default::default()
        }
    }

    pub fn build_scheme(&mut self, name: S, scheme_builder: fn(&mut Scheme<E>)) -> &mut Self {
        let mut scheme = Scheme::<E>::new();
        scheme_builder(&mut scheme);
        self.schemes.insert(name, scheme);
        self
    }

    pub fn insert_scheme(&mut self, name: S, scheme: Scheme<E>) -> Option<Scheme<E>> {
        self.schemes.insert(name, scheme)
    }

    pub fn insert_action(&mut self, name: S, event: E, action: Action) -> &mut Self {
        match self.schemes.get_mut(&name) {
            None => { bevy::log::warn!("Failed to find profile"); },
            Some(profile) => {
                profile.actions.insert(event, action);
            }
        }
        self
    }

    pub fn get_scheme(&self, name: &S) -> Option<&Scheme<E>>{
        self.schemes.get(name)
    }

    pub fn get_scheme_mut(&mut self, name: &S) -> Option<&mut Scheme<E>> {
        self.schemes.get_mut(name)
    }
}
