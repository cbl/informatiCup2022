#![feature(test)]

extern crate test;

use rstrain::model::Model;
use rstrain::tabu::TabuSearch;
use test::Bencher;

#[bench]
fn tabu_list(b: &mut Bencher) {
    let model = Model::new_for_bench();
    let mut tabu = TabuSearch::new(0, 1000, false);

    b.iter(|| tabu.search(&model));
}
