use crate::{passenger, state, train, types};
use rand::Rng;

pub fn transition(state: &mut state::State, data: &state::Data) -> () {
    let rnd = rand::thread_rng();
    let decision = rnd.gen_range(0..4);
    let t: types::Time = rnd.gen_range(0..state.p_journey.len());

    if decision == 0 && !detrain(&mut state, &data, t) {
        depart(&mut state, &data, t);
    } else if decision == 1 && !board(&mut state, &data, t) {
        depart(&mut state, &data, t);
    } else if decision == 2 && !switch_train_start(&mut state, &data, t) {
        depart(&mut state, &data, t);
    } else {
        depart(&mut state, &data, t);
    }
}

// Detrain a random person from a train
fn detrain(state: &mut state::State, data: &state::Data, t: types::Time) -> bool {
    false
}

fn board(state: &mut state::State, data: &state::Data, t: types::Time) -> bool {
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

fn switch_train_start(state: &mut state::State, data: &state::Data, t: types::Time) -> bool {
    false
}

fn depart(state: &mut state::State, data: &state::Data, t: types::Time) -> () {}
