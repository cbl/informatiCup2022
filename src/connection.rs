use crate::station::Id as SId;
use crate::types;
use std::collections::HashMap;

pub type Id = (SId, SId);
pub type Distance = i32;
pub type Connections = HashMap<Id, Connection>;

#[derive(Clone, Copy)]
pub struct Connection {
    pub name: &'static str,
    pub distance: Distance,
    pub capacity: types::Capacity,
}
