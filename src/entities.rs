use crate::connection::Connections;
use crate::passenger::{
    ArrivalTime as PArrivalTime, Location as PLocation, LocationId as PLocationId,
    LocationType as PLocationType, Passenger,
};
use crate::state::{State, States};
use crate::station::Station;
use crate::train::{
    Location as TLocation, LocationType as TLocationType, StartStationId as TStartStationId, Train,
};
use crate::types::Time;

pub struct Entities {
    pub stations: Vec<Station>,
    pub connections: Connections,
    pub trains: Vec<Train>,
    pub passengers: Vec<Passenger>,
}

impl Entities {
    pub fn latest_passenger(&self) -> PArrivalTime {
        let mut t = 0;

        for p in &self.passengers {
            if p.arrival > t {
                t = p.arrival;
            }
        }

        return t;
    }

    pub fn init_states(&self) -> States {
        let latest_arrival: Time = self.latest_passenger();

        let s_capacity = self.stations.iter().map(|s| s.capacity).collect();
        let t_capacity = self.trains.iter().map(|t| t.capacity).collect();

        let t_location = self
            .trains
            .iter()
            .map(|train| match train.start {
                TStartStationId::Nothing => TLocation {
                    typ: TLocationType::Nothing,
                    id: train.start,
                },
                _ => TLocation {
                    typ: TLocationType::Station,
                    id: train.start,
                },
            })
            .collect();

        let p_location = self
            .passengers
            .iter()
            .map(|passenger| PLocation {
                typ: PLocationType::Station,
                id: PLocationId::AnI32(passenger.start),
            })
            .collect();

        (0..latest_arrival)
            .into_iter()
            .map(|t: Time| State {
                s_capacity,
                t_capacity,
                t_location,
                p_location,
            })
            .collect()
    }
}
