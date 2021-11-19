use crate::station::Id as SId;
use crate::types;
use std::collections::HashMap;

pub type Id = (SId, SId);
pub type Name = String;
pub type Distance = f64;
pub type Connections = HashMap<Id, Connection>;

#[derive(Clone)]
pub struct Connection {
    pub name: Name,
    pub distance: Distance,
    pub capacity: types::Capacity,
}
