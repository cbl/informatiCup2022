use crate::connection::Id as CId;
use crate::entities::Entities;
use crate::passenger::{Id as PId, Location as PLocation};
use crate::solution::Solution;
use crate::station::Id as SId;
use crate::train::{Id as TId, Location as TLocation};
use crate::types::Time;
use std::fmt;

pub struct Timetable {
    pub solution: Solution,
    pub entities: Entities,
}

impl Timetable {
    // at this point: it is not checked whether the move was legal or not,
    // whenever an illegal move has been made - so when the capacity of any
    // train or station is < 0 - the cust function must weight this
    // maybe in the future: it could be needed to filter legal moves before they
    // are made

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
}

impl fmt::Display for Timetable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "TODO")
    }
}
