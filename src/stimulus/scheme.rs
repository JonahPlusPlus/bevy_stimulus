use super::*;

#[derive(Clone)]
pub struct Scheme<E> where E: Clone + Eq + std::hash::Hash {
    pub(crate) actions: HashMap<E, Action>
}

#[cfg(feature = "serialize")]
impl<E: Serialize + Clone + Eq + std::hash::Hash + std::fmt::Display> Serialize for Scheme<E> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        use ser::SerializeMap;
        let mut map = serializer.serialize_map(Some(self.actions.len()))?;
        for (k, v) in &self.actions {
            map.serialize_entry(&k.to_string(), &v)?;
        }
        map.end()
    }
}

struct SchemeVisitor<E: Clone + Eq + std::hash::Hash + std::fmt::Display> {
    marker: std::marker::PhantomData<fn() -> Scheme<E>>
}

impl<E: Clone + Eq + std::hash::Hash + std::fmt::Display>  SchemeVisitor<E> {
    fn new() -> Self {
        SchemeVisitor {
            marker: std::marker::PhantomData
        }
    }
}

impl<'de, E: Deserialize<'de> + Clone + Eq + std::hash::Hash + std::fmt::Display> serde::de::Visitor<'de> for SchemeVisitor<E> {
    type Value = Scheme<E>;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("Action HashMap")
    }

    fn visit_map<M: serde::de::MapAccess<'de>>(self, mut access: M) -> Result<Self::Value, M::Error> {
        let mut actions = HashMap::default();
        while let Some((k, v)) = access.next_entry()? {
            actions.insert(k, v);
        }
        let scheme = Scheme {
            actions
        };
        Ok(scheme)
    }
}


#[cfg(feature = "serialize")]
impl<'de, E: Deserialize<'de> + Clone + Eq + std::hash::Hash + std::fmt::Display> Deserialize<'de> for Scheme<E> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_map(SchemeVisitor::new())
    }
}

impl<E: Clone + Eq + std::hash::Hash + std::fmt::Display> Scheme<E> {
    pub fn new() -> Self {
        Self {
            actions: Default::default()
        }
    }

    pub fn insert_action(&mut self, event: E, action: Action) -> &mut Self {
        self.actions.insert(event, action);
        self
    }

    pub fn get_action(&self, event: &E) -> Option<&Action> {
        self.actions.get(event)
    }

    pub fn get_action_mut(&mut self, event: &E) -> Option<&mut Action> {
        self.actions.get_mut(event)
    }
}