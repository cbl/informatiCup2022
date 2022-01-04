use crate::model::Model;
use crate::solution::Solution;
use prettytable::{Cell, Row, Table};

pub fn debug(model: Model, solution: Solution, duration: u128, checked_moves: usize) {
    let mut table = Table::new();

    // println!("\n{}", solution.to_string(&model, true));

    table.add_row(Row::new(vec![
        Cell::new("duration"),
        Cell::new(&format!("{:.3}s", duration as f64 / 1000.0)),
    ]));
    table.add_row(Row::new(vec![
        Cell::new("compared moves"),
        Cell::new(&format!("{}", checked_moves)),
    ]));
    table.add_row(Row::new(vec![
        Cell::new("compared moves / ms"),
        Cell::new(&format!(
            "{}",
            (checked_moves as f64 / duration as f64) as i32
        )),
    ]));
    table.add_row(Row::new(vec![
        Cell::new("delays"),
        Cell::new(&format!("{}", solution.fitness())),
    ]));
    table.add_row(Row::new(vec![
        Cell::new("arrived passengers"),
        Cell::new(&format!(
            "{}/{}",
            solution.0[solution.0.len() - 1].p_arrived.len(),
            model.passengers.len()
        )),
    ]));
    table.add_row(Row::new(vec![
        Cell::new("t-max"),
        Cell::new(&format!("{}", model.t_max)),
    ]));

    table.printstd();
}
