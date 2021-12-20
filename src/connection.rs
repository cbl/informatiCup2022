use crate::station::Id as SId;
use crate::types;

pub type Id = types::Id;
pub type Name = String;
pub type Distance = f64;
pub type Connections = Vec<Connection>;

#[derive(Clone)]
pub struct Connection {
    pub name: Name,
    pub distance: Distance,
    pub capacity: types::Capacity,
    pub a: SId,
    pub b: SId,
}
