use crate::rule::{Closure, Result, Rule};

/// - Depart a train to another station when the station is full and has no passengers.
/// - Depart towards the station with the highest capacity
pub fn rules() -> Vec<Rule> {
    vec![
        // depart vs none
        Rule::IsDepartGtNone(Closure {
            c: Box::new(|a, _, state, _| {
                if state.s_capacity[a.from] == 0 && state.s_passengers[a.from].len() == 0 {
                    Result::Some(true)
                } else {
                    Result::None
                }
            }),
        }),
        // depart vs none
        Rule::IsDepartGtDepart(Closure {
            c: Box::new(|a, b, state, _| {
                Result::Some(state.s_capacity[a.to] > state.s_capacity[b.to])
            }),
        }),
    ]
}
