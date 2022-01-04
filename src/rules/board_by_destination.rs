use crate::rule::{Closure, Result, Rule};

/// Board passengers that travel the same travel path together.
pub fn rules() -> Vec<Rule> {
    vec![
        // board vs board
        Rule::IsBoardGtNone(Closure {
            c: Box::new(|a, _, state, model| {
                for p_id in state.t_passengers[a.t_id].iter() {
                    if model.passengers[a.p_id].destination == model.passengers[*p_id].destination {
                        return Result::Some(true);
                    }
                }

                Result::None
            }),
        }),
    ]
}
