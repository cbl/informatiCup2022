#![feature(test)]

extern crate test;

use rstrain::model::Model;
use rstrain::tabu::TabuGeneticSearch;
use test::Bencher;

#[bench]
fn search(b: &mut Bencher) {
    let model = Model::new_for_bench();
    let mut tabu = TabuGeneticSearch::new(0, 1000, false);

    b.iter(|| tabu.search(&model));
}
