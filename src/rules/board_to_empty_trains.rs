use crate::rule::{Closure, Result, Rule};

/// Board passengers.
pub fn rules() -> Vec<Rule> {
    vec![
        // board vs depart
        Rule::IsBoardGtDepart(Closure {
            c: Box::new(|a, _, state, model| {
                if state.t_capacity[a.t_id] == model.trains[a.t_id].capacity {
                    Result::Some(true)
                } else {
                    Result::None
                }
            }),
        }),
        // board vs none
        Rule::IsBoardGtNone(Closure {
            c: Box::new(|a, _, state, model| {
                if state.t_capacity[a.t_id] == model.trains[a.t_id].capacity {
                    Result::Some(true)
                } else {
                    Result::None
                }
            }),
        }),
    ]
}
