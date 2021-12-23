use std::cmp::Ordering;

use crate::connection::{Distance, Id as CId};
use crate::rule::{Closure, Result, Rule};

/// Board passengers that travel the same travel path together.
pub fn rules() -> Vec<Rule> {
    vec![
        // depart vs none
        Rule::IsDepartGtNone(Closure {
            c: Box::new(|a, b, state, model| {
                if state.t_passengers[a.t_id].len() == 0 {
                    return Result::None;
                }

                let a_distance: Distance = state.t_passengers[a.t_id]
                    .iter()
                    .map(|p_id| model.distance(a.to, model.passengers[*p_id].destination))
                    .sum();

                let mut connections: Vec<(CId, Distance)> = model.station_connections[a.from]
                    .iter()
                    .map(|c_id| {
                        let destination = model.get_destination(a.from, *c_id);
                        let distance = state.t_passengers[a.t_id]
                            .iter()
                            .map(|p_id| {
                                model.distance(destination, model.passengers[*p_id].destination)
                            })
                            .sum::<Distance>();

                        (*c_id, distance)
                    })
                    .collect();

                connections.sort_by(|a, b| match a.1 < b.1 {
                    true => Ordering::Less,
                    false => Ordering::Greater,
                });

                let best_connection = connections.first().unwrap();

                let delta = a_distance - best_connection.1;
                let normalized_delta = delta / best_connection.1;

                // println!("{}", normalized_delta);
                if normalized_delta > 0.7 {
                    return Result::Some(false);
                }

                Result::None
            }),
        }),
    ]
}
