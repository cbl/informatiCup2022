use crate::rule::{Closure, Result, Rule};

/// A train should depart towards the destination.
pub fn rules() -> Vec<Rule> {
    vec![
        // depart vs depart
        Rule::IsDepartGtDepart(Closure {
            c: Box::new(|a, b, state, model| {
                if state.t_passengers[a.t_id].len() == 0 {
                    return Result::None;
                }

                let p_id: usize = *state.t_passengers[a.t_id]
                    .clone()
                    .into_iter()
                    .collect::<Vec<usize>>()
                    .first()
                    .unwrap();

                let a_distance = model.distance(a.to, model.passengers[p_id].destination);
                let b_distance = model.distance(b.to, model.passengers[p_id].destination);

                Result::Some(a_distance < b_distance)
            }),
        }),
    ]
}
