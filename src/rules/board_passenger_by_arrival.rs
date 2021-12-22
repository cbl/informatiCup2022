use crate::rule::{Foo, Result, Rule, RuleNone};

fn cmp(a: Board, b: Board, state: &State, model: &Model) -> Result {
    Result::Some(model.passengers[a.p_id].arrival < model.passengers[b.p_id].arrival)
}
