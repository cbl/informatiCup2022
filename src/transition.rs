use crate::passenger::{Id as PId, Location as PLocation};
use crate::timetable::Timetable;
use crate::types::Time;
use rand::Rng;

pub trait Transition {
    fn transition(&mut self) -> ();
}
