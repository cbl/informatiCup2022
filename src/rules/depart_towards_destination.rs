use std::cmp::Ordering;

use crate::connection::Distance;
use crate::rule::{Closure, Result, Rule};

/// A train should depart towards the destination.
pub fn rules() -> Vec<Rule> {
    vec![
        // depart vs depart
        Rule::IsDepartGtDepart(Closure {
            c: Box::new(|a, b, state, model| {
                if state.t_passengers[a.t_id].len() == 0 || state.t_passengers[b.t_id].len() == 0 {
                    return Result::None;
                }

                let ps = state.t_passengers[a.t_id]
                    .clone()
                    .into_iter()
                    .collect::<Vec<usize>>();
                let a_p_id = ps.first().unwrap();

                let a_distance = model.distance(a.to, model.passengers[*a_p_id].destination);

                let ps = state.t_passengers[a.t_id]
                    .clone()
                    .into_iter()
                    .collect::<Vec<usize>>();
                let b_p_id = ps.first().unwrap();

                let b_distance = model.distance(b.to, model.passengers[*b_p_id].destination);

                // let a_sum_distance: Distance = state.t_passengers[a.t_id]
                //     .iter()
                //     .map(|p_id| model.distance(a.to, model.passengers[*p_id].destination))
                //     .sum();

                // let b_sum_distance: Distance = state.t_passengers[b.t_id]
                //     .iter()
                //     .map(|p_id| model.distance(b.to, model.passengers[*p_id].destination))
                //     .sum();

                Result::Some(a_distance < b_distance)
            }),
        }),
    ]
}
