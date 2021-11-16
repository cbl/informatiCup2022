use crate::{connection, passenger, station, train, types};
use std::fmt;

#[derive(Clone)]
pub struct State {
    pub s_capacities: Vec<Vec<station::Capacity>>,
    pub t_capacities: Vec<Vec<train::Capacity>>,
    pub t_journey: Vec<train::Journey>,
    pub p_journey: Vec<passenger::Journey>,
}

impl State {
    pub fn rounds(&self) -> usize {
        self.p_journey.len()
    }

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

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "TODO")
    }
}

pub struct Data {
    pub stations: Vec<station::Station>,
    pub connections: connection::Connections,
    pub trains: Vec<train::Train>,
    pub passengers: Vec<passenger::Passenger>,
}

impl Data {
    pub fn latest_passenger(&self) -> passenger::ArrivalTime {
        let mut t = 0;

        for p in &self.passengers {
            if p.arrival > t {
                t = p.arrival;
            }
        }

        return t;
    }
}

pub fn initial(data: Data) -> State {
    let latest_arrival = data.latest_passenger();

    // generate initial station capacities
    let s_capacities: Vec<Vec<i32>> = data
        .stations
        .iter()
        .map(|&st| (0..latest_arrival).map(|_| st.capacity).collect())
        .collect();

    // generate initial train capacities
    let t_capacities: Vec<Vec<i32>> = data
        .trains
        .iter()
        .map(|&tr| (0..latest_arrival).map(|_| tr.capacity).collect())
        .collect();

    // generate initial train journeys
    let t_journey: Vec<train::Journey> = data
        .trains
        .iter()
        .map(|tr| {
            (0..latest_arrival)
                .map(|_| match tr.start {
                    train::StartStationId::Nothing => train::Location {
                        typ: train::LocationType::Nothing,
                        id: tr.start,
                    },
                    _ => train::Location {
                        typ: train::LocationType::Station,
                        id: tr.start,
                    },
                })
                .collect()
        })
        .collect();

    // generate initial passenger journeys
    let p_journey: Vec<passenger::Journey> = data
        .passengers
        .iter()
        .map(|ps| {
            (0..latest_arrival)
                .map(|_| passenger::Location {
                    typ: passenger::LocationType::Station,
                    id: passenger::LocationId::AnI32(ps.start),
                })
                .collect()
        })
        .collect();

    return State {
        s_capacities,
        t_journey,
        t_capacities,
        p_journey,
    };
}
