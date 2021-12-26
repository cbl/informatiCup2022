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

                let mut a_nearest_p_distance: Option<Distance> = None;
                for (s_id, passengers) in state.s_passengers.iter().enumerate() {
                    for _ in passengers {
                        let distance = model.distance(s_id, a.to);
                        match a_nearest_p_distance {
                            None => a_nearest_p_distance = Some(distance),
                            Some(_d) => {
                                if distance < _d {
                                    a_nearest_p_distance = Some(distance)
                                }
                            }
                        }
                    }
                }

                let mut b_nearest_p_distance: Option<Distance> = None;
                for (s_id, passengers) in state.s_passengers.iter().enumerate() {
                    for _ in passengers {
                        let distance = model.distance(s_id, b.to);
                        match b_nearest_p_distance {
                            None => b_nearest_p_distance = Some(distance),
                            Some(_d) => {
                                if distance < _d {
                                    b_nearest_p_distance = Some(distance)
                                }
                            }
                        }
                    }
                }

                if let Some(a_distance) = a_nearest_p_distance {
                    if let Some(b_distance) = b_nearest_p_distance {
                        return Result::Some(a_distance < b_distance);
                    }
                }

                Result::None
            }),
        }),
    ]
}
