use crate::rule::{Closure, Result, Rule};

#[macro_export]
macro_rules! rule {
    ( $a:expr, $state:expr, $model:expr ) => {{
        let a_des = $model.passengers[$a.p_id].destination;
        let path_a = $model.paths.get(&($a.s_id, a_des)).unwrap();
        for p_id in $state.t_passengers[$a.t_id].iter() {
            let b_des = $model.passengers[*p_id].destination;
            let path_b = $model.paths.get(&($a.s_id, b_des)).unwrap();

            for s_id in path_a.path.iter() {
                if b_des == *s_id {
                    return Result::Some(true);
                }
            }

            for s_id in path_b.path.iter() {
                if a_des == *s_id {
                    return Result::Some(true);
                }
            }
        }

        Result::None
    }};
}

/// Board passengers that travel the same travel path together.
pub fn rules() -> Vec<Rule> {
    vec![
        // board vs depart
        Rule::IsBoardGtDepart(Closure {
            c: Box::new(|a, _, state, model| rule!(a, state, model)),
        }),
        // board vs none
        Rule::IsBoardGtNone(Closure {
            c: Box::new(|a, _, state, model| rule!(a, state, model)),
        }),
    ]
}
