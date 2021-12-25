use crate::rule::{Closure, Result, Rule};

/// Avoid station overload.
pub fn rules() -> Vec<Rule> {
    vec![
        Rule::IsDepartGtNone(Closure {
            c: Box::new(|a, _, state, model| {
                let train_arrival_time = state.t + model.train_arrival(a.t_id, a.c_id);
                let estimated_station_capacity = state.est_s_cap(train_arrival_time, a.to, model);
                let max_station_capacity = model.stations[a.to].capacity;

                if (estimated_station_capacity - 1) <= -max_station_capacity {
                    Result::Some(false)
                } else {
                    Result::None
                }
            }),
        }),
        Rule::IsStartGtNone(Closure {
            c: Box::new(|a, _, state, model| {
                if state.est_s_cap(model.t_max, a.s_id, model) <= 0 {
                    Result::Some(false)
                } else {
                    Result::None
                }
            }),
        }),
    ]
}
