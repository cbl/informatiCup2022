// pub mod arrived_passenger;
// pub mod board_passenger_by_arrival;
// pub mod station_overload;

use crate::rule::{Closure, Rule};

fn boarding() {
    let boardings = Rule::IsBoardGtBoard(Closure {
        c: Box::new(|a, b| {}),
    });
}

pub fn get_rules() {}
