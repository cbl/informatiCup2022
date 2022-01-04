use crate::types;

pub type Id = types::Id;
pub type Capacity = types::Capacity;

#[derive(Clone)]
pub struct Station {
    pub name: String,
    pub capacity: Capacity,
}
