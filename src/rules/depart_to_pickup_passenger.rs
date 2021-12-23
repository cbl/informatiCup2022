use std::cmp::Ordering;

use crate::connection::Distance;
use crate::rule::{Closure, ClosureAny, Result, Rule};
use crate::train::Location as TLocation;

/// Passengers should be picked up by the nearest train.
pub fn rules() -> Vec<Rule> {
    vec![
        Rule::IsDepartGtNone(Closure {
            c: Box::new(|a, _, state, _| {
                if state.t_passengers[a.t_id].len() == 0 && state.s_passengers[a.from].len() == 0 {
                    Result::Some(true)
                } else {
                    Result::None
                }
            }),
        }),
        // depart vs depart
        Rule::IsDepartGtDepart(Closure {
            c: Box::new(|a, b, state, model| {
                // when the train has passengers it should not pick up passengers
                // from other stations

                if state.t_passengers[a.t_id].len() > 0 {
                    return Result::None;
                }

                let mut to_pick_up = vec![];

                for (s_id, passengers) in state.s_passengers.iter().enumerate() {
                    for p_id in passengers {
                        to_pick_up.push((p_id, s_id));
                    }
                }

                to_pick_up.sort_by(|(_, a_s_id), (_, b_s_id)| {
                    match model.distance(*a_s_id, a.to) < model.distance(*b_s_id, a.to) {
                        true => Ordering::Less,
                        false => Ordering::Greater,
                    }
                });

                if to_pick_up.len() == 0 {
                    return Result::None;
                }

                let a_nearest_p = to_pick_up.first().unwrap();

                to_pick_up.sort_by(|(_, a_s_id), (_, b_s_id)| {
                    match model.distance(*a_s_id, b.to) < model.distance(*b_s_id, b.to) {
                        true => Ordering::Less,
                        false => Ordering::Greater,
                    }
                });

                let a_nearest_p = to_pick_up.first().unwrap();
                let b_nearest_p = to_pick_up.first().unwrap();

                let a_distance: Distance = model.distance(a.to, a_nearest_p.1);

                let b_distance: Distance = model.distance(b.to, b_nearest_p.1);

                // println!(
                //     "({:?}) {} < {} ({:?})",
                //     a_nearest_p, a_distance, b_distance, b_nearest_p
                // );

                Result::Some(a_distance < b_distance)
                // Result::None

                // find empty train that is the closest to the nearest passenger
                // let nearest_t = state
                //     .t_capacity
                //     .iter()
                //     .enumerate()
                //     .filter(|(t_id, cap)| cap == model.trains[*t_id].capacity)
                //     .filter_map(|(t_id, cap)| {
                //         if let TLocation::Station(s_id) = state.t_location[t_id] {
                //             Some((t_id, s_id))
                //         } else {
                //             None
                //         }
                //     })
                //     .collect()
                //     .sort_by(|(_, a_s_id), (_, b_s_id)| {
                //         model.distance(a_s_id, nearest_p) < model.distance(b_s_id, nearest_p)
                //     })
                //     .first()
                //     .unwrap();

                // if nearest_t == a.t_id {
                //     return Result::Some(true);
                // }

                // Result::None
            }),
        }),
    ]
}
