use crate::passenger::{Id as PId, Location as PLocation};
use crate::timetable::Timetable;
use crate::types::Time;
use rand::Rng;

pub fn transition(timetable: &mut Timetable) -> () {
    let mut rnd = rand::thread_rng();
    let decision = rnd.gen_range(0..4);
    let t: Time = rnd.gen_range(0..timetable.solution.0.len());

    if decision == 0 && !detrain(&mut timetable, t) {
        depart(&mut timetable, t);
    } else if decision == 1 && !board(&mut timetable, t) {
        depart(&mut timetable, t);
    } else if decision == 2 && !switch_train_start(&mut timetable, t) {
        depart(&mut timetable, t);
    } else {
        depart(&mut timetable, t);
    }
}

// Detrain a random person from a train
fn detrain(tt: &mut Timetable, t: Time) -> bool {
    false
}

fn board(tt: &mut Timetable, t: Time) -> bool {
    let mut rnd = rand::thread_rng();

    // filter train locations for trains that are located in a station
    let t_locations = tt.solution.0[t]
        .t_location
        .into_iter()
        .filter(|&l| l.is_station());

    // filter passengers locations for passengers that are located in a station
    // where also the station has a train with enough capacity
    let p_locations: Vec<PLocation> = tt.solution.0[t]
        .p_location
        .into_iter()
        .filter(|&l| l.is_station())
        .filter(|&pl| t_locations.any(|tl| tl.matches_passenger_station(&pl)))
        .collect();

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
            tt.board(p_id, t_id, t);
            true
        }
    }
}

fn switch_train_start(tt: &mut Timetable, t: Time) -> bool {
    // TODO
    false
}

fn depart(tt: &mut Timetable, t: Time) -> () {
    // TODO
}
