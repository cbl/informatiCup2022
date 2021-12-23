use crate::model::Model;
use crate::move_::{Board, Depart, Detrain, None, TrainStart};
use crate::rule::{Result, Rule, RuleNone};
use crate::state::State;

pub struct StationOverload();
impl RuleNone<Board, Depart> for StationOverload {}
impl RuleNone<Board, Detrain> for StationOverload {}
impl RuleNone<Board, TrainStart> for StationOverload {}
impl RuleNone<Board, None> for StationOverload {}
impl RuleNone<Depart, Depart> for StationOverload {}
impl RuleNone<Depart, Detrain> for StationOverload {}
impl RuleNone<Depart, TrainStart> for StationOverload {}
impl RuleNone<Depart, None> for StationOverload {}
impl RuleNone<Detrain, Detrain> for StationOverload {}
impl RuleNone<Detrain, TrainStart> for StationOverload {}
impl RuleNone<Detrain, None> for StationOverload {}
impl RuleNone<TrainStart, TrainStart> for StationOverload {}
impl RuleNone<TrainStart, None> for StationOverload {}

impl Rule<Board, Board> for StationOverload {
    fn cmp(a: Board, b: Board, state: &State, model: &Model) -> Result {
        Result::None
    }
}
