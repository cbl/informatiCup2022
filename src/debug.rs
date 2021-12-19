use crate::model::Model;
use crate::solution::Solution;
use prettytable::{Cell, Row, Table};

pub fn debug(model: Model, solution: Solution, duration: u128) {
    // Create the table
    let mut table = Table::new();

    println!("\n{}", solution.to_string(&model, true));

    table.add_row(Row::new(vec![
        Cell::new("duration"),
        Cell::new(&format!("{:.3}s", duration as f64 / 1000.0)),
    ]));
    table.add_row(Row::new(vec![
        Cell::new("delays"),
        Cell::new(&format!("{}", solution.fitness())),
    ]));
    table.add_row(Row::new(vec![
        Cell::new("fitness"),
        Cell::new(&format!("{}", solution.state_fitness(&model))),
    ]));
    table.add_row(Row::new(vec![
        Cell::new("arrived passengers"),
        Cell::new(&format!(
            "{}/{}",
            solution.0[solution.0.len() - 1].arrived_passengers().len(),
            model.passengers.len()
        )),
    ]));

    table.printstd();
}
