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
        //         let t_passengers = state.t_passengers.iter().map(|p| p.len()).sum::<usize>();
        //         let s_passengers = state.s_passengers.iter().map(|p| p.len()).sum::<usize>();
        //         if t_passengers > 0 || s_passengers == 0 {
        //             return Result::None;
        //         }

        //         return Result::None;
        //     }),
        // }),
    ]
}
