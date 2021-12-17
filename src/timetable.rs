use crate::connection::{Connection, Id as CId};
use crate::model::Model;
use crate::passenger::{Capacity, Id as PId, Location as PLocation};
use crate::solution::Solution;
use crate::state::{Boarding, Departure, Detrain, Start};
use crate::station::Id as SId;
use crate::train::{Id as TId, Location as TLocation, StartStation, Train};
use crate::types::Time;
use rand::prelude::ThreadRng;
use rand::seq::SliceRandom;
use rand::Rng;
use std::fmt;

/// A timetable holds information about the model - stations, connections,
/// trains and passengers - and the solution which stores the state
/// of each entity at any given point in time.
#[derive(Clone)]
pub struct Timetable {
    pub model: Model,
    pub solution: Solution,
    rnd: ThreadRng,
}

impl Timetable {
    // at this point: it is not checked whether the move was legal or not,
    // whenever an illegal move has been made - so when the capacity of any
    // train or station is < 0 - the cust function must weight this
    // maybe in the future: it could be needed to filter legal moves before they
    // are made

    /// Create a new Timetable instance.
    pub fn new(model: Model, solution: Solution) -> Timetable {
        Timetable {
            model,
            solution,
            rnd: rand::thread_rng(),
        }
    }

    /// Execute a transition.
    ///
    /// A transition performes one of the following moves randomly for a random
    /// point in time:
    /// - depart
    /// - board
    /// - detrain
    /// - switch_train_start
    ///
    /// Sometimes a move cannot be performed, e.g.: there are no passengers on
    /// any station that can be boarded to a train. For that case a random train
    /// is be departed.
    pub fn transition(&mut self) -> () {
        let decision = self.rnd.gen_range(0..4);
        let t: Time = self.rnd.gen_range(0..self.solution.0.len());

        if decision == 0 && !self.detrain_random(t) {
            self.depart_random(t);
        } else if decision == 1 && !self.board_random(t) {
            self.depart_random(t);
        } else if decision == 2 && !self.switch_random_train_start() {
            self.depart_random(t);
        } else {
            self.depart_random(t);
        }
    }

    /// Board a passenger to a given train at the given point in time.
    pub fn board(&mut self, p_id: PId, t_id: TId, t: Time) -> () {
        for i in t..self.solution.0.len() {
            // increase capacity of the previous location of the passenger
            self.increase_passenger_location_capacity(p_id, i);

            // Update passenger location
            self.solution.0[i].p_location[p_id] = PLocation::Train(t_id);

            // decrease capacity of the boarded train
            self.solution.0[i].t_capacity[t_id] -= self.model.passengers[p_id].size;
        }
    }

    /// Detrain a passenger from a given train to a station at the given point
    /// in time.
    pub fn detrain(&mut self, p_id: PId, t_id: TId, s_id: SId, t: Time) -> () {
        for i in t..self.solution.0.len() {
            // increase capacity of the previous location of the passenger
            self.increase_passenger_location_capacity(p_id, i);

            // update passenger location

            self.solution.0[i].p_location[p_id] =
                match self.model.passengers[p_id].destination == s_id {
                    true => PLocation::Arrived,
                    false => {
                        // decrease the capacity of the station when the passenger has not arrived yet.
                        self.solution.0[i].s_capacity[s_id] -= self.model.passengers[p_id].size;
                        PLocation::Station(s_id)
                    }
                };

            // TODO: is it needed to increase the capacity of the station after
            // the passenger has arrived?
        }
    }

    /// Depart a train from a station at the given point in time.
    pub fn depart(&mut self, t_id: TId, c_id: CId, t: Time) -> bool {
        if self.model.connections.contains_key(&c_id) {
            self.undo_train_journey(t_id, t);
        }

        if let Some(connection) = self.model.connections.get(&c_id) {
            for i in t..self.solution.0.len() {
                // Zeiteinheiten * Geschwindigkeit >= Streckenlänge
                // https://github.com/informatiCup/informatiCup2022/issues/7
                if ((i - t) as f64 * self.model.trains[t_id].speed) >= connection.distance {
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
                        .get_mut(&connection.name)
                        .unwrap() -= 1;
                }
            }

            return true;
        }

        false
    }

    fn undo_passenger_journey(&mut self, p_id: PId, location: PLocation, t: Time) {
        for i in t..self.solution.0.len() {
            self.increase_passenger_location_capacity(p_id, i);
            self.solution.0[t].p_location[p_id] = location;
            self.decrease_passenger_location_capacity(p_id, i);
        }
    }

    /// Undoing a train journey from the given point in time to the end. This
    /// includes undoing all passenger journeys that have boarded to the train
    /// in the timespan.
    fn undo_train_journey(&mut self, t_id: TId, t: Time) {
        for i in t..self.solution.0.len() {
            // unboard all passengers
            (0..self.solution.0[i].p_location.len()).for_each(|p_id| {
                if let Boarding::Some(boarding) = self.solution.boarding_at(p_id, t) {
                    if boarding.1 == t_id {
                        self.undo_passenger_journey(p_id, PLocation::Station(boarding.0), i);
                    }
                }
            });

            // increase capacity of previous train location
            self.increase_train_location_capacity(t_id, i);
        }
    }

    /// Detrain a random passenger.
    ///
    /// Returns false when no passenger is on a train that is on a station that
    /// has capacity left.
    pub fn detrain_random(&mut self, t: Time) -> bool {
        if t == 2 {
            return false;
        }
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
            .filter(|(p_id, l)| match l {
                PLocation::Train(t_id) => {
                    for i in 1..t {
                        if let Boarding::Some(boarding) = self.solution.boarding_at(*p_id, t - i) {
                            return match self.solution.0[t].t_location[*t_id] {
                                TLocation::Station(s_id) => boarding.0 != s_id,
                                _ => false,
                            };
                        }
                    }

                    false
                }
                _ => false,
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
        if t == 0 {
            return false;
        }

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
            .filter(|(p_id, pl)| {
                trains_iterator.clone().any(|(t_id, tl)| {
                    tl.matches_passenger_station(&pl)
                        && self.solution.0[t].t_capacity[t_id] >= self.model.passengers[*p_id].size
                })
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
        if t == self.solution.0.len() - 1 || t == 0 {
            return false;
        }

        // get iteration containing trains that are located at a station
        let trains_iterator = self.solution.0[t]
            .t_location
            .clone()
            .into_iter()
            .enumerate()
            .filter(|(_, l)| l.is_station());
        // .filter(|(t_id, _)| self.solution.0[t + 1].t_location[*t_id].is_station());

        // find a random connection that match the location of any train in a station
        // and has capacity greater that 0.
        let connections = self
            .model
            .connections
            .clone()
            .into_iter()
            .enumerate()
            .filter(|(_, (_, c))| self.solution.0[t].c_capacity[&c.name] > 0)
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
                self.depart(*t_id, *c_id, t + 1);
                return true;
            }
        }

        false
    }

    pub fn switch_random_train_start(&mut self) -> bool {
        let trains = self
            .model
            .trains
            .clone()
            .into_iter()
            .enumerate()
            .filter(|(_, t)| t.start == StartStation::Any)
            .collect::<Vec<(TId, Train)>>();

        let stations = self.solution.0[0]
            .s_capacity
            .clone()
            .into_iter()
            .enumerate();
        // .filter(|(_, c)| c > 0);

        if let Some((t_id, _)) = trains.choose(&mut self.rnd) {
            if let Some((s_id, _)) = stations
                .collect::<Vec<(SId, Capacity)>>()
                .choose(&mut self.rnd)
            {
                self.undo_train_journey(*t_id, 0);
                (0..self.solution.0.len())
                    .for_each(|t| self.solution.0[t].t_location[*t_id] = TLocation::Station(*s_id))
            }
        }

        false
    }

    pub fn previous_train_station(&self, t_id: TId, t: Time) -> Option<SId> {
        for i in (1..t) {
            if let TLocation::Station(s_id) = self.solution.0[t - i].t_location[t_id] {
                return Some(s_id);
            }
        }

        None
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

    fn increase_passenger_location_capacity(&mut self, p_id: PId, t: Time) {
        match self.solution.0[t].p_location[p_id] {
            PLocation::Train(t_id) => {
                self.solution.0[t].t_capacity[t_id] += self.model.passengers[p_id].size
            }
            PLocation::Station(s_id) => {
                self.solution.0[t].s_capacity[s_id] += self.model.passengers[p_id].size;
            }
            _ => {}
        };
    }

    fn decrease_passenger_location_capacity(&mut self, p_id: PId, t: Time) {
        match self.solution.0[t].p_location[p_id] {
            PLocation::Train(t_id) => {
                self.solution.0[t].t_capacity[t_id] -= self.model.passengers[p_id].size
            }
            PLocation::Station(s_id) => {
                self.solution.0[t].s_capacity[s_id] -= self.model.passengers[p_id].size;
            }
            _ => {}
        };
    }

    fn increase_train_location_capacity(&mut self, t_id: TId, t: Time) {
        match self.solution.0[t].t_location[t_id] {
            TLocation::Connection(prev_c_id) => {
                if let Some(connection) = self.model.connections.get(&prev_c_id) {
                    *self.solution.0[t]
                        .c_capacity
                        .get_mut(&connection.name)
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
        self.model
            .trains
            .iter()
            .enumerate()
            .for_each(|(t_id, train)| {
                writeln!(f, "[Train:{}]", train.name);
                self.solution.0.iter().enumerate().for_each(|(t, s)| {
                    if let Start::Station(s_id) = self.solution.t_start_at(t_id, t) {
                        writeln!(f, "{} Start {}", t, self.model.stations[s_id].name);
                    }

                    if let Departure::Connection(c_id) = self.solution.departure_at(t_id, t) {
                        if let Option::Some(connection) = self.model.connections.get(&c_id) {
                            writeln!(f, "{} Depart {}", t, connection.name);
                        }
                    }
                });
                writeln!(f, "");
            });

        // add passenger journeys
        self.model
            .passengers
            .iter()
            .enumerate()
            .for_each(|(p_id, passenger)| {
                writeln!(f, "[Passenger:{}]", passenger.name);
                self.solution.0.iter().enumerate().for_each(|(t, s)| {
                    if let Boarding::Some((_, t_id)) = self.solution.boarding_at(p_id, t) {
                        writeln!(f, "{} Board {}", t, self.model.trains[t_id].name);
                    } else if self.solution.detrain_at(p_id, t) == Detrain::Ok {
                        writeln!(f, "{} Detrain", t);
                    }
                });
                writeln!(f, "");
            });

        Ok(())
    }
}
