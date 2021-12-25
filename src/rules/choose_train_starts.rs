use crate::rule::{Closure, Result, Rule};
use crate::types::Time;

/// Choose the starting position for a train.
/// The best starting position is give by:
/// - the station has passengers
/// - the station is with the least sum(p.arrival)
pub fn rules() -> Vec<Rule> {
    vec![
        // start vs none
        Rule::IsStartGtNone(Closure {
            c: Box::new(|a, _, state, model| {
                if state.s_passengers[a.s_id]
                    .iter()
                    .any(|p_id| model.passengers[*p_id].size <= state.t_capacity[a.t_id])
                {
                    Result::Some(true)
                } else {
                    Result::None
                }
            }),
        }),
        // depart vs none
        Rule::IsStartGtStart(Closure {
            c: Box::new(|a, _, state, model| {
                let a_sum_arrival = state.s_passengers[a.s_id]
                    .iter()
                    .filter(|&p_id| model.passengers[*p_id].size <= state.t_capacity[a.t_id])
                    .map(|p_id| model.passengers[*p_id].arrival)
                    .sum::<Time>();

                let b_sum_arrival = state.s_passengers[a.s_id]
                    .iter()
                    .filter(|&p_id| model.passengers[*p_id].size <= state.t_capacity[a.t_id])
                    .map(|p_id| model.passengers[*p_id].arrival)
                    .sum::<Time>();

                Result::Some(a_sum_arrival < b_sum_arrival)
            }),
        }),
    ]
}
