use crate::station::Id as SId;
use crate::types;
use rust_decimal::Decimal;

pub type Id = types::Id;
pub type Name = String;
pub type Distance = Decimal;
pub type Connections = Vec<Connection>;

/// A connection between station a and station b.
#[derive(Clone)]
pub struct Connection {
    /// The name of the connection.
    pub name: Name,

    /// The distance of the connection.
    pub distance: Distance,

    /// The capacity determines how many trains can travel on it at the same
    /// time.
    pub capacity: types::Capacity,

    pub a: SId,
    pub b: SId,
}
