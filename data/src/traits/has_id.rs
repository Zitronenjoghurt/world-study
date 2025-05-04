use serde::{Deserialize, Serialize};
use std::hash::Hash;

pub trait HasId {
    type Id: Hash + Eq + Serialize + for<'de> Deserialize<'de>;
    fn id(&self) -> Self::Id;
    fn with_id(self, id: Self::Id) -> Self;
}
