use crate::rule::{Closure, ClosureAny, Result, Rule};

#[macro_export]
macro_rules! dap_rule {
    ( $a:expr, $model:expr ) => {{
        if $a.s_id == $model.passengers[$a.p_id].destination {
            Result::Some(true)
        } else {
            Result::None
        }
    }};
}

/// Passengers should detrain when their train has arrived in the corresponding
/// destination station.
pub fn rules() -> Vec<Rule> {
    vec![
        Rule::IsBoardGtDetrain(Closure {
            c: Box::new(|_, b, _, model| dap_rule!(b, model)),
        }),
        Rule::IsDetrainGtDepart(Closure {
            c: Box::new(|a, _, _, model| dap_rule!(a, model)),
        }),
        Rule::IsDetrainGtNone(Closure {
            c: Box::new(|a, _, _, model| dap_rule!(a, model)),
        }),
    ]
}
