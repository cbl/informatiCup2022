use crate::rule::{Closure, ClosureAny, Result, Rule};

/// Passengers should detrain when their train has arrived in the corresponding
/// destination station.
pub fn rules() -> Vec<Rule> {
    vec![
        // detrain vs any
        Rule::IsDetrainGtAny(ClosureAny {
            c: Box::new(|a, _, model| {
                if a.s_id == model.passengers[a.p_id].destination {
                    Result::Some(true)
                } else {
                    Result::None
                }
            }),
        }),
    ]
}
