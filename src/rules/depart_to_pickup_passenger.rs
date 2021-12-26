use crate::connection::Distance;
use crate::rule::{Closure, Result, Rule};

/// Passengers should be picked up by the nearest train.
pub fn rules() -> Vec<Rule> {
    vec![
        Rule::IsDepartGtNone(Closure {
            c: Box::new(|a, _, state, _| {
                if state.t_passengers[a.t_id].len() > 0 || state.s_passengers[a.from].len() > 0 {
                    return Result::None;
                } else { 
                    Result::Some(state.s_passengers.iter().map(|p| p.len()).sum::<usize>() > 0)
                }
            }),
        }),
        // depart vs depart
        Rule::IsDepartGtDepart(Closure {
            c: Box::new(|a, b, state, model| {
                Result::None
            }),
        }),
    ]
}
