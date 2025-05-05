use crate::traits::has_id::HasId;
use serde::{Deserialize, Serialize};
use std::collections::hash_map::Keys;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct DataMap<T>
where
    T: HasId + Serialize + for<'de> Deserialize<'de>,
    T::Id: Serialize + for<'de> Deserialize<'de>,
{
    entities: HashMap<T::Id, Arc<T>>,
}

impl<T> DataMap<T>
where
    T: HasId + Serialize + for<'de> Deserialize<'de>,
    T::Id: Serialize + for<'de> Deserialize<'de>,
{
    pub fn new() -> Self {
        Self {
            entities: HashMap::new(),
        }
    }

    pub fn keys(&self) -> Keys<T::Id, Arc<T>> {
        self.entities.keys()
    }

    pub fn add(&mut self, entity: T) {
        let id = entity.id();
        self.entities.insert(id, Arc::new(entity));
    }

    pub fn get<Q>(&self, id: &Q) -> Option<&Arc<T>>
    where
        T::Id: std::borrow::Borrow<Q>,
        Q: std::hash::Hash + Eq + ?Sized,
    {
        self.entities.get(id)
    }

    pub fn iter(&self) -> impl Iterator<Item = &Arc<T>> {
        self.entities.values()
    }
}

impl<T> Serialize for DataMap<T>
where
    T: HasId + Serialize + for<'de> Deserialize<'de>,
    T::Id: Serialize + for<'de> Deserialize<'de>,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(Some(self.entities.len()))?;
        for (k, v) in &self.entities {
            map.serialize_entry(k, v.as_ref())?;
        }
        map.end()
    }
}

impl<'de, T> Deserialize<'de> for DataMap<T>
where
    T: HasId + Serialize + for<'d> Deserialize<'d>,
    T::Id: Serialize + for<'d> Deserialize<'d>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let temp_map: HashMap<T::Id, T> = HashMap::deserialize(deserializer)?;

        let entities = temp_map
            .into_iter()
            .map(|(k, v)| (k, Arc::new(v)))
            .collect();

        Ok(DataMap { entities })
    }
}
