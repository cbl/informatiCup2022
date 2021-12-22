use crate::model::Model;
use crate::move_::{Board, Depart, Detrain, None, TrainStart};
use crate::rule::{False, Rule};
use crate::state::State;

pub struct ArrivedPassener();
impl False<Board, Depart> for ArrivedPassener {}
impl False<Board, Detrain> for ArrivedPassener {}
impl False<Board, TrainStart> for ArrivedPassener {}
impl False<Board, None> for ArrivedPassener {}
impl False<Depart, Depart> for ArrivedPassener {}
impl False<Depart, Detrain> for ArrivedPassener {}
impl False<Depart, TrainStart> for ArrivedPassener {}
impl False<Depart, None> for ArrivedPassener {}
impl False<Detrain, Detrain> for ArrivedPassener {}
impl False<Detrain, TrainStart> for ArrivedPassener {}
impl False<Detrain, None> for ArrivedPassener {}
impl False<TrainStart, TrainStart> for ArrivedPassener {}
impl False<TrainStart, None> for ArrivedPassener {}

impl Rule<Board, Board> for ArrivedPassener {
    fn cmp(a: Board, b: Board, state: &State, model: &Model) -> bool {
        false
    }
}
