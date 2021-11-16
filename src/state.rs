use crate::{connection, passenger, station, train, types};
use crate::types::Capacity;
use crate::train::Location as TLocation;
use crate::passenger::Location as PLocation;

pub type States = Vec<State>;

#[derive(Clone)]
pub struct State {
    pub s_capacity: Vec<Capacity>,
    pub t_capacity: Vec<Capacity>,
    pub t_location: Vec<TLocation>,
    pub p_location: Vec<PLocation>,
}

impl State {

    pub fn board(&self, p_id: passenger::Id, t_id: train::Id, t: types::Time) -> () {
        for i in t + 1..self.rounds() {
            self.p_journey[p_id].locations[i] = passenger::Location {
                typ: passenger::LocationType::Train,
                id: passenger::LocationId::AnI32(t_id),
            }
        }
    }

    pub fn detrain(&self, p_id: passenger::Id, s_id: station::Id, t: types::Time) -> () {
        for i in t..self.rounds() {
            // TODO: needs data
            self.p_journey[p_id].locations[i] = passenger::Location {
                typ: passenger::LocationType::Station,
                id: passenger::LocationId::AnI32(s_id),
            }
        }
    }

    pub fn depart(&self, t_id: train::Id, c_id) -> () {

    }

    pub fn passenger_delay(&self, data: &Data) -> f64 {
        let mut delay = 0.0;
        for (p_id, journey) in self.p_journey.iter().enumerate() {
            for (t, location) in journey.locations.iter().enumerate() {
                if t > data.passengers[p_id].arrival {
                    delay += 1.0;
                }
            }
        }
        return delay;
    }
}
