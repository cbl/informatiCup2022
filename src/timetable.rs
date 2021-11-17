use crate::connection::Id as CId;
use crate::entities::Entities;
use crate::passenger::{Id as PId, Location as PLocation};
use crate::solution::Solution;
use crate::state::State;
use crate::station::Id as SId;
use crate::train::{Id as TId, Location as TLocation, Train};
use crate::types::Time;
use rand::Rng;
use std::fmt;

pub struct Timetable {
    pub entities: Entities,
    pub solution: Solution,
}

impl Timetable {
    // at this point: it is not checked whether the move was legal or not,
    // whenever an illegal move has been made - so when the capacity of any
    // train or station is < 0 - the cust function must weight this
    // maybe in the future: it could be needed to filter legal moves before they
    // are made

    pub fn transition(&mut self) -> () {
        let mut rnd = rand::thread_rng();
        let decision = rnd.gen_range(0..4);
        let t: Time = rnd.gen_range(0..self.solution.0.len());

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
            self.increase_location_capacity(p_id, t);

            // Update passenger location
            self.solution.0[i].p_location[p_id] = PLocation::Train(t_id);

            // decrease capacity of the boarded train
            self.solution.0[i].t_capacity[t_id] -= self.entities.passengers[p_id].size;
        }
    }

    pub fn detrain(&mut self, p_id: PId, s_id: SId, t: Time) -> () {
        for i in t..self.solution.0.len() {
            // increase capacity of the previous location of the passenger
            self.increase_location_capacity(p_id, t);

            // update passenger location
            self.solution.0[i].p_location[p_id] = match self.entities.passengers[p_id].start == s_id
            {
                true => PLocation::Arrived,
                false => PLocation::Station(s_id),
            };

            if self.entities.passengers[p_id].start != s_id {
                // decrease capacity of the new station when the passenger has
                // not arrived yet
                self.solution.0[i].s_capacity[s_id] -= self.entities.passengers[p_id].size;
            }

            // TODO: is it needed to increase the capacity of the station after
            // the passenger has arrived?
        }
    }

    pub fn depart(&mut self, t_id: TId, c_id: CId, t: Time) -> () {
        self.solution.0[t + 1].t_location[t_id] = TLocation::Connection(c_id);
        // TODO:
    }

    fn increase_location_capacity(&mut self, p_id: PId, t: Time) -> () {
        let size = self.entities.passengers[p_id].size;
        // Todo
    }

    fn decrease_location_capacity(&mut self, p_id: PId, t: Time) -> () {
        let size = self.entities.passengers[p_id].size;
        // Todo
    }

    pub fn detrain_random(&mut self, t: Time) -> bool {
        false
    }

    pub fn board_random(&mut self, t: Time) -> bool {
        let mut rnd = rand::thread_rng();

        // filter train locations for trains that are located in a station
        let t_locations = self.solution.0[t]
            .t_location
            .clone()
            .into_iter()
            .filter(|&l| l.is_station());

        // filter passengers locations for passengers that are located in a station
        // where also the station has a train with enough capacity
        let p_locations: Vec<PLocation> = self.solution.0[t]
            .p_location
            .clone()
            .into_iter()
            .filter(|l| l.is_station())
            .filter(|pl| {
                t_locations
                    .clone()
                    .any(|tl| tl.matches_passenger_station(&pl))
            })
            .collect::<Vec<PLocation>>();

        // return false when no passenger where found that is on a station with
        // an available train.
        if p_locations.len() == 0 {
            return false;
        }

        // choosing a random passanger
        let p_id: PId = rnd.gen_range(0..p_locations.len());

        // find the first train that is on the same station
        let train = t_locations
            .filter(|&tl| tl.matches_passenger_station(&p_locations[p_id]))
            .enumerate()
            .next();

        match train {
            None => false,
            Some((t_id, _)) => {
                self.board(p_id, t_id, t);
                true
            }
        }
    }

    pub fn depart_random(&mut self, t: Time) -> bool {
        false
    }

    pub fn switch_random_train_start(&mut self, t: Time) -> bool {
        false
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
                    if self.solution.t_start_at(t_id, t) {
                    } else if self.solution.departs_at(t_id, t) {
                        writeln!(f, "{} Depart", t);
                    }
                });
            });

        // add passenger journeys
        self.entities
            .passengers
            .iter()
            .enumerate()
            .for_each(|(p_id, passenger)| {
                writeln!(f, "[Passenger:{}]", passenger.name);
                self.solution.0.iter().enumerate().for_each(|(t, s)| {
                    if self.solution.boards_at(p_id, t) {
                        writeln!(f, "{} Board", t);
                    } else if self.solution.detrains_at(p_id, t) {
                        writeln!(f, "{} Detrain", t);
                    }
                });
            });

        Ok(())
    }
}
