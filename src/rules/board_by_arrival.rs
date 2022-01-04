use crate::rule::{Closure, Result, Rule};

/// Passengers with an early arrival are boarded first.
pub fn rules() -> Vec<Rule> {
    vec![
        // board vs board
        Rule::IsBoardGtBoard(Closure {
            c: Box::new(|a, b, _, model| {
                Result::Some(model.passengers[a.p_id].arrival < model.passengers[b.p_id].arrival)
            }),
        }),
    ]
}
