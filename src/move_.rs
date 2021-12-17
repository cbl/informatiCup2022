use crate::connection::Id as CId;
use crate::model::Model;
use crate::passenger::Id as PId;
use crate::station::Id as SId;
use crate::train::Id as TId;

#[derive(Hash, Clone, PartialEq, Copy)]
pub enum Move {
    Board(TId, PId, SId),
    Detrain(TId, PId, SId),
    Depart(TId, SId, CId),
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
            Move::Depart(t_id, s_id, c_id) => {
                if let Some(connection) = model.connections.get(c_id) {
                    return format!(
                        "Depart {} to {} from {}",
                        model.trains[*t_id].name, connection.name, model.stations[*s_id].name,
                    )
                    .to_owned();
                } else {
                    return "".to_owned();
                }
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
