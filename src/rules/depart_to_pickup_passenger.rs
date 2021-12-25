use std::cmp::Ordering;

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
                // when the train has passengers it should not pick up passengers
                // from other stations

                if state.t_passengers[a.t_id].len() > 0 {
                    return Result::None;
                }

                // let mut to_pick_up = vec![];
                let mut a_nearest_p_distance: Option<Distance> = None;
                for (s_id, passengers) in state.s_passengers.iter().enumerate() {
                    for p_id in passengers {
                        let distance = model.distance(s_id, a.to);
                        match a_nearest_p_distance {
                            None => a_nearest_p_distance = Some(distance),
                            Some(_d) => {
                                if distance < _d {
                                    a_nearest_p_distance = Some(distance)
                                }
                            }
                        }
                        // to_pick_up.push((p_id, s_id));
                    }
                }

                let mut b_nearest_p_distance: Option<Distance> = None;
                for (s_id, passengers) in state.s_passengers.iter().enumerate() {
                    for p_id in passengers {
                        let distance = model.distance(s_id, b.to);
                        match b_nearest_p_distance {
                            None => b_nearest_p_distance = Some(distance),
                            Some(_d) => {
                                if distance < _d {
                                    b_nearest_p_distance = Some(distance)
                                }
                            }
                        }
                        // to_pick_up.push((p_id, s_id));
                    }
                }

                // to_pick_up.sort_by(|(_, a_s_id), (_, b_s_id)| {
                //     match model.distance(*a_s_id, a.to) < model.distance(*b_s_id, a.to) {
                //         true => Ordering::Less,
                //         false => Ordering::Greater,
                //     }
                // });

                // if to_pick_up.len() == 0 {
                //     return Result::None;
                // }

                // let a_nearest_p = to_pick_up.first().unwrap();

                // let mut to_pick_up = to_pick_up.clone();

                // to_pick_up.sort_by(|(_, a_s_id), (_, b_s_id)| {
                //     match model.distance(*a_s_id, b.to) < model.distance(*b_s_id, b.to) {
                //         true => Ordering::Less,
                //         false => Ordering::Greater,
                //     }
                // });

                // let b_nearest_p = to_pick_up.first().unwrap();

                if let Some(a_distance) = a_nearest_p_distance {
                    if let Some(b_distance) = b_nearest_p_distance {
                        return Result::Some(a_distance < b_distance);
                    }
                }

                // let a_distance: Distance = model.distance(a.to, a_nearest_p.1);

                // let b_distance: Distance = model.distance(b.to, b_nearest_p.1);

                // println!(
                //     "({:?}) {} < {} ({:?})",
                //     a_nearest_p, a_distance, b_distance, b_nearest_p
                // );

                // Result::Some(a_distance < b_distance)
                Result::None
            }),
        }),
    ]
}
