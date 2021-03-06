use crate::rule::{Closure, Result, Rule};

use rust_decimal::Decimal;

/// A train should depart to the exact destination.
pub fn rules() -> Vec<Rule> {
    vec![
        // depart vs depart
        Rule::IsDepartGtDepart(Closure {
            c: Box::new(|a, b, state, model| {
                if state.t_passengers[a.t_id]
                    .iter()
                    .map(|p_id| model.distance(a.to, model.passengers[*p_id].destination))
                    .any(|d| d == Decimal::ZERO)
                {
                    return Result::Some(true);
                }

                if state.t_passengers[b.t_id]
                    .iter()
                    .map(|p_id| model.distance(b.to, model.passengers[*p_id].destination))
                    .any(|d| d == Decimal::ZERO)
                {
                    return Result::Some(false);
                }

                Result::None
            }),
        }),
    ]
}
