use crate::types;

pub type Id = types::Id;
pub type Speed = f64;
pub type Capacity = types::Capacity;

pub struct Station {
    // id: Id,
    pub name: &'static str,
    pub capacity: Capacity,
}
