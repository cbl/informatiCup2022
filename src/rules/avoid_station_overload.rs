use crate::rule::{Closure, Result, Rule};

/// Avoid station overload.
pub fn rules() -> Vec<Rule> {
    vec![
        Rule::IsDepartGtNone(Closure {
            c: Box::new(|a, _, state, model| {
                let est_s_cap = state.est_s_cap(model.t_max, a.to, model);
                let s_cap = model.stations[a.to].capacity;

                if (est_s_cap - 1) <= -s_cap {
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
