#![feature(test)]

extern crate test;

use fxhash::hash64;
use rstrain::model::Model;
use rstrain::move_::Move;
use test::Bencher;

#[bench]
fn clone(b: &mut Bencher) {
    let state = Model::new_for_bench().initial_state();

    b.iter(|| state.clone());
}

#[bench]
fn get_moves(b: &mut Bencher) {
    let model = Model::new_for_bench();
    let state = model.initial_state();

    b.iter(|| {
        for t_id in 0..model.trains.len() {
            state.get_moves(t_id, &model);
        }
    });
}

#[bench]
fn hash(b: &mut Bencher) {
    let model = Model::new_for_bench();
    let state = model.initial_state();

    b.iter(|| hash64(&state));
}

#[bench]
fn push(b: &mut Bencher) {
    let model = Model::new_for_bench();
    let mut state = model.initial_state();
    let moves: Vec<Move> = (0..model.trains.len())
        .map(|t_id| state.get_moves(t_id, &model))
        .flatten()
        .collect();

    b.iter(|| {
        for m in moves.iter() {
            state.push(*m, &model);
        }
    });
}
