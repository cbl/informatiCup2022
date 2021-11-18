use crate::passenger::{Id as PId, Location as PLocation};
use crate::state::{Boarding, Departure, Detrain, Start, State};
use crate::train::{Id as TId, Location as TLocation};
use crate::types::Time;

/// The soltion holds a list of states for all entities at any given point in
/// time.
#[derive(Clone)]
pub struct Solution(pub Vec<State>);

impl Solution {
    /// Gets the start of a train at the given time.
    ///
    /// A train starts at a station when the current location of the train is of
    /// type `train::Location::Statio(s_id)` and the location for `t-1` is
    /// `train::Location::Nothing`.
    pub fn t_start_at(&self, t_id: TId, t: Time) -> Start {
        match self.0[t].t_location[t_id] {
            TLocation::Station(s_id) => {
                if t == 0 {
                    Start::Station(s_id)
                } else {
                    match self.0[t - 1].t_location[t_id] {
                        TLocation::Nothing => Start::Station(s_id),
                        _ => Start::Nothing,
                    }
                }
            }
            _ => Start::Nothing,
        }
    }

    /// Gets the departure of a train at the given time.
    pub fn departure_at(&self, t_id: TId, t: Time) -> Departure {
        if t == 0 {
            return Departure::Nothing;
        }

        match self.0[t].t_location[t_id] {
            TLocation::Connection(c_id) => match self.0[t - 1].t_location[t_id] {
                TLocation::Station(_) => Departure::Connection(c_id),
                _ => Departure::Nothing,
            },
            _ => Departure::Nothing,
        }
    }

    /// Gets the boarding of a passenger at the given time.
    pub fn boarding_at(&self, p_id: PId, t: Time) -> Boarding {
        if t == 0 {
            return Boarding::Nothing;
        }

        match self.0[t].p_location[p_id] {
            PLocation::Train(t_id) => match self.0[t - 1].p_location[p_id] {
                PLocation::Station(_) => Boarding::Train(t_id),
                _ => Boarding::Nothing,
            },
            _ => Boarding::Nothing,
        }
    }

    /// Gets the detrain of a passenger at the given time.
    pub fn detrain_at(&self, p_id: PId, t: Time) -> Detrain {
        Detrain::Nothing
    }
}
