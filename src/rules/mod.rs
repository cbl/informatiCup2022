mod avoid_station_overload;
mod board_by_destination;
mod board_by_travel_path;
mod board_passenger_by_arrival;
mod board_to_empty_trains;
mod choose_train_starts;
mod depart_passenger_trains;
mod depart_to_exact_destination;
mod depart_towards_destination;
mod detrain_arrived_passenger;
mod free_up_space;

use crate::rule::Rule;

pub fn get_rules() -> Vec<Rule> {
    vec![
        // rules are applied in the following order:
        //
        avoid_station_overload::rules(),
        //
        detrain_arrived_passenger::rules(),
        //
        board_passenger_by_arrival::rules(),
        //
        board_by_destination::rules(),
        //
        board_by_travel_path::rules(),
        //
        board_to_empty_trains::rules(),
        //
        depart_to_exact_destination::rules(),
        //
        depart_towards_destination::rules(),
        //
        depart_passenger_trains::rules(),
        //
        free_up_space::rules(),
        //
        choose_train_starts::rules(),
    ]
    .into_iter()
    .flatten()
    .collect()
}
