use crate::connection::Id as CId;
use crate::model::Model;
use crate::passenger::Id as PId;
use crate::rule::Result;
use crate::state::State;
use crate::station::Id as SId;
use crate::train::Id as TId;

pub trait MoveTr {}

/// Board move
#[derive(Hash, Clone, PartialEq, Copy)]
pub struct Board {
    /// The id of the train
    pub t_id: TId,
    /// The id of the passenger being boarded
    pub p_id: PId,
    /// The id of the station from where the passenger is boarded
    pub s_id: SId,
}

impl MoveTr for Board {}

impl Board {
    /// Gets the string representation of the move for the given model.
    pub fn to_string(&self, model: &Model) -> String {
        format!(
            "Board {} to {} from {}",
            model.passengers[self.p_id].name,
            model.trains[self.t_id].name,
            model.stations[self.s_id].name,
        )
    }
}

/// Detrain move
#[derive(Hash, Clone, PartialEq, Copy)]
pub struct Detrain {
    /// The id of the train
    pub t_id: TId,
    /// The id of the passenger being detrained
    pub p_id: PId,
    /// The id of the station where the passenger has arrived
    pub s_id: SId,
}

impl MoveTr for Detrain {}

impl Detrain {
    /// Gets the string representation of the move for the given model.
    pub fn to_string(&self, model: &Model) -> String {
        format!(
            "Detrain {} from {} to {}",
            model.passengers[self.p_id].name,
            model.trains[self.t_id].name,
            model.stations[self.s_id].name,
        )
    }
}

#[derive(Hash, Clone, PartialEq, Copy)]
pub struct Depart {
    /// The id of the departing train
    pub t_id: TId,
    /// The id of the station from where the train departs
    pub from: SId,
    /// The id of the **destination** station
    pub to: SId,
    /// The id of connection
    pub c_id: CId,
}

impl MoveTr for Depart {}

impl Depart {
    /// Gets the string representation of the move for the given model.
    pub fn to_string(&self, model: &Model) -> String {
        format!(
            "Depart {} from {} to {} via {}",
            model.trains[self.t_id].name,
            model.stations[self.from].name,
            model.stations[self.to].name,
            model.connections[self.c_id].name,
        )
    }
}

/// Train start move
#[derive(Hash, Clone, PartialEq, Copy)]
pub struct Start {
    /// The id of the starting train
    pub t_id: TId,
    /// The if of the start station
    pub s_id: SId,
}

impl MoveTr for Start {}

impl Start {
    /// Gets the string representation of the move for the given model.
    pub fn to_string(&self, model: &Model) -> String {
        format!(
            "Start {} on {}",
            model.trains[self.t_id].name, model.stations[self.s_id].name,
        )
    }
}

/// No move.
#[derive(Hash, Clone, PartialEq, Copy)]
pub struct None();

impl MoveTr for None {}

impl None {
    /// Gets the string representation of the move for the given model.
    pub fn to_string(&self, _: &Model) -> String {
        "".to_string()
    }
}

/// Enum wrapper for all possible moves.
#[derive(Hash, Clone, PartialEq, Copy)]
pub enum Move {
    Board(Board),
    Detrain(Detrain),
    Depart(Depart),
    Start(Start),
    None(None),
}

impl Move {
    /// Determines whether a move is greater than another move.
    pub fn is_gt(&self, m: &Move, state: &State, model: &Model) -> bool {
        for rule in &model.rules {
            if let Result::Some(result) = rule.is_gt(self, m, state, model) {
                return result;
            }
        }

        false
    }

    /// Gets the string representation of the move for the given model.
    pub fn to_string(&self, model: &Model) -> String {
        match self {
            Move::Board(m) => m.to_string(model),
            Move::Detrain(m) => m.to_string(model),
            Move::Depart(m) => m.to_string(model),
            Move::Start(m) => m.to_string(model),
            Move::None(m) => m.to_string(model),
        }
    }
}
