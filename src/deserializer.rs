use super::{Config, Story};

/// Trait for extract config/story from MessagePack bytes.
pub trait Deserializable {
    fn deserialize(bytes: &[u8]) -> Self
    where
        Self: Sized;
}

impl Deserializable for Config {
    fn deserialize(bytes: &[u8]) -> Self {
        rmp_serde::from_slice(bytes).unwrap()
    }
}

impl Deserializable for Story {
    fn deserialize(bytes: &[u8]) -> Self {
        rmp_serde::from_slice(bytes).unwrap()
    }
}
