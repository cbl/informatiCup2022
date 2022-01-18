#![feature(test)]

extern crate test;

use rstrain::model::Model;
use rstrain::move_::Move;
use test::Bencher;

#[bench]
fn rules(b: &mut Bencher) {
    let model = Model::new_for_bench();
    let state = model.initial_state();
    let moves: Vec<Move> = (0..model.trains.len())
        .map(|t_id| state.get_moves(t_id, &model))
        .flatten()
        .collect();

    b.iter(|| {
        for a in moves.iter() {
            for b in moves.iter() {
                a.is_gt(b, &state, &model);
            }
        }
    });
}
