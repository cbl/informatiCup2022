use crate::rule::{Closure, Result, Rule};

/// Passengers should be picked up by a train.
pub fn rules() -> Vec<Rule> {
    vec![
        Rule::IsDepartGtNone(Closure {
            c: Box::new(|a, _, state, _| {
                if state.t_passengers[a.t_id].len() > 0 || state.s_passengers[a.from].len() > 0 {
                    Result::None
                } else {
                    Result::Some(true)
                }
            }),
        }),
        // Rule::IsDepartGtDepart(Closure {
        //     c: Box::new(|a, b, state, model| {
        //         for closest_station in &model.closest_stations[a.from] {
        //             if state.s_passengers[closest_station.s_id].len() == 0 {
        //                 continue;
        //             }

        //             let a_distance = model.distance(a.to, closest_station.s_id);
        //             let b_distance = model.distance(b.to, closest_station.s_id);

        //             return Result::Some(a_distance < b_distance);
        //         }

        //         Result::None
        //     }),
        // }),
    ]
}
