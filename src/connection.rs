use crate::station::Id as SId;

pub type Id = (SId, SId);
pub type Distance = i32;
pub type Connections = Vec<Vec<Distance>>;
