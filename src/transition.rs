use crate::timetable::Timetable;
use crate::{passenger, state, train, types};
use rand::Rng;

pub fn transition(tt: &mut Timetable) -> () {
    let rnd = rand::thread_rng();
    let decision = rnd.gen_range(0..4);
    let t: types::Time = rnd.gen_range(0..tt.states.len());

    if decision == 0 && !detrain(&mut tt, t) {
        depart(&mut tt, t);
    } else if decision == 1 && !board(&mut tt, t) {
        depart(&mut tt, t);
    } else if decision == 2 && !switch_train_start(&mut tt, t) {
        depart(&mut tt, t);
    } else {
        depart(&mut tt, t);
    }
}

// Detrain a random person from a train
fn detrain(tt: &mut Timetable, t: types::Time) -> bool {
    false
}

fn board(tt: &mut Timetable, t: types::Time) -> bool {
    let rnd = rand::thread_rng();

    // Filter passengers that are located in a station
    let p_journeys = state
        .p_journey
        .into_iter()
        .filter(|&journey| matches!(journey.at(t).typ, passenger::LocationType::Station));

    let t_journeys = state
        .t_journey
        .into_iter()
        .filter(|&journey| matches!(journey.at(t).typ, train::LocationType::Station))
        .filter(|&tj| {
            tj.locations
                .iter()
                .any(|&tl| p_journeys.any(|pj| pj.locations.iter().any(|&pl| tl.id == pl.id)))
        });

    let p_id: passenger::Id = rnd.gen_range(0..p_journeys.count());
    let tr = t_journeys
        .filter(|&tj| {
            tj.locations.iter().any(|&tl| {
                p_journeys.collect::<Vec<passenger::Journey>>()[p_id]
                    .locations
                    .iter()
                    .any(|&pl| tl.id == pl.id)
            })
        })
        .enumerate()
        .next();

    match tr {
        None => false,
        Some((t_id, _)) => {
            state.board(p_id, t_id, t);
            true
        }
    }
}

fn switch_train_start(tt: &mut Timetable, t: types::Time) -> bool {
    // TODO
    false
}

fn depart(tt: &mut Timetable, t: types::Time) -> () {
    // TODO
}
