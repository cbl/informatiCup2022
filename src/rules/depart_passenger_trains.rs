use crate::rule::{Closure, ClosureAny, Result, Rule};

/// Trains with passengers should depart.
pub fn rules() -> Vec<Rule> {
    vec![
        // depart vs none
        Rule::IsDepartGtNone(Closure {
            c: Box::new(|a, _, state, model| {
                if state.t_capacity[a.t_id] < model.trains[a.t_id].capacity {
                    Result::Some(true)
                } else {
                    Result::None
                }
            }),
        }),
    ]
}
