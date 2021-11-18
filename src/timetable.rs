use crate::connection::{Connection, Id as CId};
use crate::entities::Entities;
use crate::passenger::{Id as PId, Location as PLocation};
use crate::solution::Solution;
use crate::state::{Boarding, Departure, Detrain, Start, State};
use crate::station::Id as SId;
use crate::train::{Id as TId, Location as TLocation, Train};
use crate::types::Time;
use rand::prelude::ThreadRng;
use rand::seq::SliceRandom;
use rand::Rng;
use std::collections::HashMap;
use std::fmt;
use std::iter::Filter;

/// A timetable holds information about all existing entities - stations,
/// connections, trains and passengers - and the solution which stores the state
/// of each entity at any given point in time.
pub struct Timetable {
    pub entities: Entities,
    pub solution: Solution,
    rnd: ThreadRng,
}

impl Timetable {
    // at this point: it is not checked whether the move was legal or not,
    // whenever an illegal move has been made - so when the capacity of any
    // train or station is < 0 - the cust function must weight this
    // maybe in the future: it could be needed to filter legal moves before they
    // are made

    pub fn new(entities: Entities, solution: Solution) -> Timetable {
        Timetable {
            entities,
            solution,
            rnd: rand::thread_rng(),
        }
    }

    pub fn transition(&mut self) -> () {
        let decision = self.rnd.gen_range(0..4);
        let t: Time = self.rnd.gen_range(0..self.solution.0.len());

        if decision == 0 && !self.detrain_random(t) {
            self.depart_random(t);
        } else if decision == 1 && !self.board_random(t) {
            self.depart_random(t);
        } else if decision == 2 && !self.switch_random_train_start(t) {
            self.depart_random(t);
        } else {
            self.depart_random(t);
        }
    }

    pub fn board(&mut self, p_id: PId, t_id: TId, t: Time) -> () {
        for i in t + 1..self.solution.0.len() {
            // increase capacity of the previous location of the passenger
            self.increase_p_location_capacity(p_id, i);

            // Update passenger location
            self.solution.0[i].p_location[p_id] = PLocation::Train(t_id);

            // decrease capacity of the boarded train
            self.solution.0[i].t_capacity[t_id] -= self.entities.passengers[p_id].size;
        }
    }

    pub fn detrain(&mut self, p_id: PId, t_id: TId, s_id: SId, t: Time) -> () {
        for i in t..self.solution.0.len() {
            // increase capacity of the previous location of the passenger
            self.increase_p_location_capacity(p_id, i);

            // update passenger location
            self.solution.0[i].p_location[p_id] = match self.entities.passengers[p_id].start == s_id
            {
                true => PLocation::Arrived,
                false => {
                    // decrease the capacity of the station when the passenger has not arrived yet.
                    self.solution.0[i].s_capacity[s_id] -= self.entities.passengers[p_id].size;
                    PLocation::Station(s_id)
                }
            };

            // TODO: is it needed to increase the capacity of the station after
            // the passenger has arrived?
        }
    }

    pub fn depart(&mut self, t_id: TId, c_id: CId, t: Time) -> bool {
        self.solution.0[t + 1].t_location[t_id] = TLocation::Connection(c_id);

        match self.entities.connections.get(&c_id) {
            None => false,
            Some(connection) => {
                for i in t..self.solution.0.len() {
                    // increase capacities of train locations that are
                    // overridden by the new connection and the destination
                    match self.solution.0[t].t_location[t_id] {
                        TLocation::Connection(prev_c_id) => {
                            if let Some(connection) = self.entities.connections.get(&prev_c_id) {
                                *self.solution.0[t]
                                    .c_capacity
                                    .get_mut(connection.name)
                                    .unwrap() += 1;
                            }
                        }
                        TLocation::Station(s_id) => {
                            self.solution.0[t].s_capacity[s_id] += 1;
                        }
                        _ => {}
                    };

                    // Zeiteinheiten * Geschwindigkeit >= Streckenlänge
                    // https://github.com/informatiCup/informatiCup2022/issues/7
                    if ((i - t) as f64 * self.entities.trains[t_id].speed) >= connection.distance {
                        // update train location to the destination
                        self.solution.0[i].t_location[t_id] = TLocation::Station(c_id.1);
                        // decrease station capacity
                        self.solution.0[i].s_capacity[c_id.1] -= 1;
                    } else {
                        // update train location to the new connection
                        self.solution.0[i].t_location[t_id] = TLocation::Connection(c_id);
                        // decrease connection capacity
                        *self.solution.0[i]
                            .c_capacity
                            .get_mut(connection.name)
                            .unwrap() -= 1;
                    }
                }

                true
            }
        }
    }

    /// Detrain a random passenger.
    ///
    /// Returns false when no passenger is on a train that is on a station that
    /// has capacity left.
    pub fn detrain_random(&mut self, t: Time) -> bool {
        // get iteration containing trains that are located at a station
        let trains_iterator = self.solution.0[t]
            .t_location
            .clone()
            .into_iter()
            .enumerate()
            .filter(|(_, l)| l.is_station());

        let passengers = self.solution.0[t]
            .p_location
            .clone()
            .into_iter()
            .enumerate()
            .filter(|(_, l)| {
                trains_iterator
                    .clone()
                    .any(|(t_id, _)| l.matches_train(t_id))
            })
            .collect::<Vec<(PId, PLocation)>>();

        match passengers.choose(&mut self.rnd) {
            Some((p_id, location)) => match location {
                PLocation::Train(t_id) => match self.solution.0[t].t_location[*t_id] {
                    TLocation::Station(s_id) => {
                        self.detrain(*p_id, *t_id, s_id, t);
                        true
                    }
                    _ => false,
                },
                _ => false,
            },
            // return false when no passenger where found that is on a station with
            // an available train.
            None => false,
        }
    }

    /// Board a random passenger to the a train.
    ///
    /// A random passenger will be boarded that is on a station with a train
    /// that has capacity left for the passenger.
    ///
    /// Returns false when no passenger can be boarded.
    pub fn board_random(&mut self, t: Time) -> bool {
        // filter train locations for trains that are located in a station
        let trains_iterator = self.solution.0[t]
            .t_location
            .clone()
            .into_iter()
            .enumerate()
            .filter(|(_, l)| l.is_station());

        // filter passengers locations for passengers that are located in a station
        // where also the station has a train with enough capacity
        let passengers = self.solution.0[t]
            .p_location
            .clone()
            .into_iter()
            .enumerate()
            .filter(|(_, l)| l.is_station())
            .filter(|(_, pl)| {
                trains_iterator
                    .clone()
                    .any(|(_, tl)| tl.matches_passenger_station(&pl))
            })
            .collect::<Vec<(PId, PLocation)>>();

        match passengers.choose(&mut self.rnd) {
            Some((p_id, location)) => {
                // find a random train that is on the same station
                let trains = trains_iterator
                    .filter(|(_, tl)| tl.matches_passenger_station(&location))
                    .collect::<Vec<(TId, TLocation)>>();

                match trains.choose(&mut self.rnd) {
                    Some((t_id, _)) => {
                        self.board(*p_id, *t_id, t);
                        true
                    }
                    None => false,
                }
            }
            // return false when no passenger where found that is on a station with
            // an available train.
            None => false,
        }
    }

    pub fn depart_random(&mut self, t: Time) -> bool {
        // get iteration containing trains that are located at a station
        let trains_iterator = self.solution.0[t]
            .t_location
            .clone()
            .into_iter()
            .enumerate()
            .filter(|(_, l)| l.is_station());

        // find a random connection that match the location of any train in a station
        // and has capacity greater that 0.
        let connections = self
            .entities
            .connections
            .clone()
            .into_iter()
            .enumerate()
            .filter(|(_, (_, c))| self.solution.0[t].c_capacity[c.name] > 0)
            .filter(|(_, ((s1_id, _), _))| {
                trains_iterator.clone().any(|(_, location)| match location {
                    TLocation::Station(s_id) => s_id == *s1_id,
                    _ => false,
                })
            })
            .collect::<Vec<(usize, (CId, Connection))>>();

        if let Some((_, (c_id, c))) = connections.choose(&mut self.rnd) {
            // find a random train that is on the same station
            let trains = trains_iterator
                .filter(|(_, tl)| match tl {
                    TLocation::Station(s_id) => *s_id == c_id.0,
                    _ => false,
                })
                .collect::<Vec<(TId, TLocation)>>();

            if let Some((t_id, _)) = trains.choose(&mut self.rnd) {
                self.depart(*t_id, *c_id, t);
                return true;
            }
        }

        false
    }

    pub fn switch_random_train_start(&mut self, t: Time) -> bool {
        false
    }

    /// Gets an iterator that filteret all trains that are located in a station.
    fn trains_in_station(&self, t: Time) -> impl Iterator<Item = (TId, TLocation)> {
        self.solution.0[t]
            .t_location
            .clone()
            .into_iter()
            .enumerate()
            .filter(|(_, l)| l.is_station())
    }

    fn increase_p_location_capacity(&mut self, p_id: PId, t: Time) {
        match self.solution.0[t].p_location[p_id] {
            PLocation::Train(t_id) => {
                self.solution.0[t].t_capacity[t_id] += self.entities.passengers[p_id].size
            }
            PLocation::Station(s_id) => {
                self.solution.0[t].s_capacity[s_id] += self.entities.passengers[p_id].size;
            }
            _ => {}
        };
    }

    fn increase_t_location_capacity(&mut self, t_id: TId, t: Time) {
        match self.solution.0[t].t_location[t_id] {
            TLocation::Connection(prev_c_id) => {
                if let Some(connection) = self.entities.connections.get(&prev_c_id) {
                    *self.solution.0[t]
                        .c_capacity
                        .get_mut(connection.name)
                        .unwrap() += 1;
                }
            }
            TLocation::Station(s_id) => {
                self.solution.0[t].s_capacity[s_id] += 1;
            }
            _ => {}
        };
    }
}

impl fmt::Display for Timetable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // add train journeys
        self.entities
            .trains
            .iter()
            .enumerate()
            .for_each(|(t_id, train)| {
                writeln!(f, "[Train:{}]", train.name);
                self.solution.0.iter().enumerate().for_each(|(t, s)| {
                    if let Start::Station(s_id) = self.solution.t_start_at(t_id, t) {
                        writeln!(f, "{} Start {}", t, self.entities.stations[s_id].name);
                    }

                    if let Departure::Connection(c_id) = self.solution.departure_at(t_id, t) {
                        if let Option::Some(connection) = self.entities.connections.get(&c_id) {
                            writeln!(f, "{} Depart {}", t, connection.name);
                        }
                    }
                });
                writeln!(f, "");
            });

        // add passenger journeys
        self.entities
            .passengers
            .iter()
            .enumerate()
            .for_each(|(p_id, passenger)| {
                writeln!(f, "[Passenger:{}]", passenger.name);
                self.solution.0.iter().enumerate().for_each(|(t, s)| {
                    if let Boarding::Train(t_id) = self.solution.boarding_at(p_id, t) {
                        writeln!(f, "{} Board {}", t, self.entities.trains[t_id].name);
                    } else if self.solution.detrain_at(p_id, t) == Detrain::Ok {
                        writeln!(f, "{} Detrain", t);
                    }
                });
                writeln!(f, "");
            });

        Ok(())
    }
}
