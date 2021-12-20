use crate::connection::Id as CId;
use crate::model::Model;
use crate::passenger::Id as PId;
use crate::station::Id as SId;
use crate::train::Id as TId;

#[derive(Hash, Clone, PartialEq, Copy)]
pub enum Move {
    /// Board move
    /// TId - the id of the train
    /// PId - the id of the passenger being boarded
    /// SId - the id of the station from where the passenger is boarded
    Board(TId, PId, SId),

    /// Detrain move
    /// TId - the id of the train
    /// PId - the id of the passenger being detrained
    /// SId - the id of the station where the passenger has arrived
    Detrain(TId, PId, SId),

    /// Depart move
    /// TId - the id of the departing train
    /// SId - the id of the **destination** station
    /// CId - the id of connection
    Depart(TId, (SId, SId), CId),

    /// TrainStart move
    /// TId - the id of the starting train
    /// SId - the if of the start station
    TrainStart(TId, SId),
}

impl Move {
    pub fn to_string(&self, model: &Model) -> String {
        match self {
            Move::Board(t_id, p_id, s_id) => {
                format!(
                    "Board {} to {} from {}",
                    model.passengers[*p_id].name,
                    model.trains[*t_id].name,
                    model.stations[*s_id].name,
                )
            }
            Move::Detrain(t_id, p_id, s_id) => {
                format!(
                    "Detrain {} from {} to {}",
                    model.passengers[*p_id].name,
                    model.trains[*t_id].name,
                    model.stations[*s_id].name,
                )
            }
            Move::Depart(t_id, (from, to), c_id) => {
                format!(
                    "Depart {} from {} to {} via {}",
                    model.trains[*t_id].name,
                    model.stations[*from].name,
                    model.stations[*to].name,
                    model.connections[*c_id].name,
                )
            }
            Move::TrainStart(t_id, s_id) => {
                format!(
                    "Start {} on {}",
                    model.trains[*t_id].name, model.stations[*s_id].name,
                )
            }
        }
    }
}
