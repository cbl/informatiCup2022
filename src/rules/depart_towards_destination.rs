use crate::connection::Distance;
use crate::rule::{Closure, Result, Rule};

/// A train should depart towards the destination.
pub fn rules() -> Vec<Rule> {
    vec![
        // depart vs depart
        Rule::IsDepartGtDepart(Closure {
            c: Box::new(|a, b, state, model| {
                let a_sum_distance: Distance = state.t_passengers[a.t_id]
                    .iter()
                    .map(|p_id| model.distance(a.to, model.passengers[*p_id].destination))
                    .sum();

                let b_sum_distance: Distance = state.t_passengers[b.t_id]
                    .iter()
                    .map(|p_id| model.distance(b.to, model.passengers[*p_id].destination))
                    .sum();

                Result::Some(a_sum_distance < b_sum_distance)
            }),
        }),
    ]
}
