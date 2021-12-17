use crate::connection::Id as CId;
use crate::passenger::Id as PId;
use crate::station::Id as SId;
use crate::train::Id as TId;

#[derive(Hash, Clone, PartialEq)]
pub enum Move {
    Board(TId, PId),
    Detrain(TId, PId),
    Depart(TId, SId, CId),
}
