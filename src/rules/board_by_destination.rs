use crate::rule::{Closure, Result, Rule};

/// Board passengers that travel the same travel path together.
pub fn rules() -> Vec<Rule> {
    vec![
        // board vs depart
        Rule::IsBoardGtBoard(Closure {
            c: Box::new(|a, b, state, model| {
                if model.passengers[a.p_id].destination == model.passengers[b.p_id].destination {
                    Result::Some(true)
                } else {
                    Result::None
                }
            }),
        }),
    ]
}
