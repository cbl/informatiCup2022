use crate::connection::Id as CId;
use crate::model::Model;
use crate::passenger::Id as PId;
use crate::rule::{Result, Rule};
use crate::rules;
use crate::state::State;
use crate::station::Id as SId;
use crate::train::Id as TId;

pub trait MoveStruct {}
pub trait BoardTr {}

#[derive(Hash, Clone, PartialEq, Copy)]
pub struct Board {
    pub t_id: TId,
    pub p_id: PId,
    pub s_id: SId,
}
impl MoveStruct for Board {}
impl BoardTr for Board {}
impl MoveStruct for BoardTr {}

#[derive(Hash, Clone, PartialEq, Copy)]
pub struct Detrain {
    pub t_id: TId,
    pub p_id: PId,
    pub s_id: SId,
}
impl MoveStruct for Detrain {}

#[derive(Hash, Clone, PartialEq, Copy)]
pub struct Depart {
    pub t_id: TId,
    pub from: SId,
    pub to: SId,
    pub c_id: CId,
}
impl MoveStruct for Depart {}

#[derive(Hash, Clone, PartialEq, Copy)]
pub struct Start {
    pub t_id: TId,
    pub s_id: SId,
}
impl MoveStruct for Start {}

#[derive(Hash, Clone, PartialEq, Copy)]
pub struct None();
impl MoveStruct for None {}

#[derive(Hash, Clone, PartialEq, Copy)]
pub enum Move {
    /// Board move
    /// TId - the id of the train
    /// PId - the id of the passenger being boarded
    /// SId - the id of the station from where the passenger is boarded
    Board(Board),

    /// Detrain move
    /// TId - the id of the train
    /// PId - the id of the passenger being detrained
    /// SId - the id of the station where the passenger has arrived
    Detrain(Detrain),

    /// Depart move
    /// TId - the id of the departing train
    /// SId - the id of the **destination** station
    /// CId - the id of connection
    Depart(Depart),

    /// Start move
    /// TId - the id of the starting train
    /// SId - the if of the start station
    Start(Start),

    None(None),
}

impl Move {
    pub fn is_gt(&self, m: &Move, state: &State, model: &Model) -> bool {
        false
        // cmp_rules!(
        //     self,
        //     m,
        //     state,
        //     model,
        //     // rules::station_overload::StationOverload,
        //     // rules::arrived_passenger::ArrivedPassenger,
        //     // rules::board_passenger_by_arrival::BoardPassener
        // )
    }

    pub fn to_string(&self, model: &Model) -> String {
        match self {
            Move::Board(m) => {
                format!(
                    "Board {} to {} from {}",
                    model.passengers[m.p_id].name,
                    model.trains[m.t_id].name,
                    model.stations[m.s_id].name,
                )
            }
            Move::Detrain(m) => {
                format!(
                    "Detrain {} from {} to {}",
                    model.passengers[m.p_id].name,
                    model.trains[m.t_id].name,
                    model.stations[m.s_id].name,
                )
            }
            Move::Depart(m) => {
                format!(
                    "Depart {} from {} to {} via {}",
                    model.trains[m.t_id].name,
                    model.stations[m.from].name,
                    model.stations[m.to].name,
                    model.connections[m.c_id].name,
                )
            }
            Move::TrainStart(m) => {
                format!(
                    "Start {} on {}",
                    model.trains[m.t_id].name, model.stations[m.s_id].name,
                )
            }
            _ => "".to_string(),
        }
    }
}

// fn is_board_gt_board(a: &Board, b: &Board, state: &State, model: &Model) -> bool {
//     model.passengers[a.1].arrival < model.passengers[b.1].arrival
// }
// fn is_board_gt_detrain(a: &Board, b: &Detrain, state: &State, model: &Model) -> bool {
//     b.2 != model.passengers[b.1].destination
// }
// fn is_board_gt_depart(a: &Board, b: &Depart, state: &State, model: &Model) -> bool {
//     // AVOID_OVERLOAD: should depart when the station will be overloaded next move
//     if state.est_s_cap(state.t + 1, a.2, model) < 0 {
//         return false;
//     }

//     if state.t_passengers[a.0].len() == 0 {
//         return true;
//     }

//     // board passengers that need to travel the same path
//     let a_des = model.passengers[a.1].destination;
//     let path_a = model.paths.get(&(a.2, a_des)).unwrap();
//     for p_id in state.t_passengers[a.0].iter() {
//         let b_des = model.passengers[*p_id].destination;
//         let path_b = model.paths.get(&(a.2, b_des)).unwrap();

//         for s_id in path_a.path.iter() {
//             if a_des == *s_id {
//                 return true;
//             }
//         }

//         for s_id in path_b.path.iter() {
//             if b_des == *s_id {
//                 return true;
//             }
//         }
//     }

//     return false;
// }

// fn is_board_gt_none(a: &Board, state: &State, model: &Model) -> bool {
//     true
//     // todo t.destinations < 2
// }

// fn is_detrain_gt_detrain(a: &Detrain, b: &Detrain, state: &State, model: &Model) -> bool {
//     if state.t_passengers[a.0].len() == 0 || state.t_passengers[b.0].len() == 0 {
//         return false;
//     }

//     let a_arrival = state.t_passengers[a.0]
//         .iter()
//         .map(|p_id| model.passengers[*p_id].arrival)
//         .sum::<Time>();

//     let b_arrival = state.t_passengers[b.0]
//         .iter()
//         .map(|p_id| model.passengers[*p_id].arrival)
//         .sum::<Time>();

//     a_arrival < b_arrival
// }

// fn is_detrain_gt_depart(a: &Detrain, b: &Depart, state: &State, model: &Model) -> bool {
//     // AVOID_OVERLOAD: should depart when the station will be overloaded next move
//     if state.est_s_cap(state.t + 1, a.2, model) < 0 {
//         return false;
//     }

//     a.2 == model.passengers[a.1].destination
// }

// fn is_detrain_gt_none(a: &Detrain, state: &State, model: &Model) -> bool {
//     a.2 == model.passengers[a.1].destination
// }

// fn is_depart_gt_depart(a: &Depart, b: &Depart, state: &State, model: &Model) -> bool {
//     if state.t_passengers[a.0].len() == 0 && state.t_passengers[b.0].len() == 0 {
//         if state.s_passengers[a.1 .1].len() == 0 {
//             return false;
//         }
//         if state.s_passengers[b.1 .1].len() == 0 {
//             return true;
//         }

//         // todo: dont move to many trains toward the station

//         // when both trains are empty:
//         // head empty trains to the closest stations that has passengers
//         // return model.distance(a.1 .0, a.1 .1) < model.distance(b.1 .0, b.1 .1);
//     }

//     let mut a_distance = 0.0;
//     for p_id in &state.t_passengers[a.0] {
//         let d = model.distance(a.1 .1, model.passengers[*p_id].destination);

//         if d == 0.0 {
//             return true;
//         }

//         a_distance += d;
//     }

//     let mut b_distance = 0.0;
//     for p_id in &state.t_passengers[b.0] {
//         let d = model.distance(b.1 .1, model.passengers[*p_id].destination);

//         if d == 0.0 {
//             return true;
//         }

//         b_distance += d;
//     }

//     if a_distance == b_distance {
//         return state.c_capacity[a.2] > state.c_capacity[b.2];
//     }

//     a_distance < b_distance
// }

// fn is_depart_gt_none((t_id, (from, to), c_id): &Depart, state: &State, model: &Model) -> bool {
//     // stations needs to have at least one free spot to be able to depart a
//     // train to the station.
//     if state.est_s_cap(model.train_arrival(*t_id, *c_id), *to, model) + model.stations[*to].capacity
//         <= 1
//     {
//         return false;
//     }

//     // station capacity = 0
//     if state.s_capacity[*from] == 0 {
//         return true;
//     }

//     // train has passengers
//     if state.t_passengers[*t_id].len() > 0 {
//         return true;
//     }

//     // cannot pickup passengers here but at destination
//     if state.s_passengers[*from].len() == 0 && state.s_passengers[*to].len() > 0 {
//         return true;
//     }

//     let a_cap = state.s_capacity[*from] as f64 / model.stations[*from].capacity as f64;
//     let b_cap = state.s_capacity[*from] as f64 / model.stations[*to].capacity as f64;
//     let c_cap = state.c_capacity[*c_id] as f64 / model.connections[*c_id].capacity as f64;

//     // destination has more space and connection is relatively empty
//     a_cap < 0.3 && b_cap > 0.7 && c_cap > 0.8
// }

// fn is_train_start_gt_train_start(
//     a: &TrainStart,
//     b: &TrainStart,
//     state: &State,
//     model: &Model,
// ) -> bool {
//     state.s_passengers[a.1].len() > state.s_passengers[b.1].len()
// }

// impl Move {
//     /// Determines wethere the move is greater than another one.
//     pub fn is_gt(&self, m: &Move, state: &State, model: &Model) -> bool {
//         match self {
//             Move::Board(a) => match m {
//                 Move::Board(b) => is_board_gt_board(a, b, state, model),
//                 Move::Detrain(b) => is_board_gt_detrain(a, b, state, model),
//                 Move::Depart(b) => is_board_gt_depart(a, b, state, model),
//                 Move::None => is_board_gt_none(a, state, model),
//                 _ => true,
//             },
//             Move::Detrain(a) => match m {
//                 Move::Board(b) => !is_board_gt_detrain(b, a, state, model),
//                 Move::Detrain(b) => is_detrain_gt_detrain(a, b, state, model),
//                 Move::Depart(b) => is_detrain_gt_depart(a, b, state, model),
//                 Move::None => is_detrain_gt_none(a, state, model),
//                 _ => true,
//             },
//             Move::Depart(a) => match m {
//                 Move::Board(b) => !is_board_gt_depart(b, a, state, model),
//                 Move::Detrain(b) => !is_detrain_gt_depart(b, a, state, model),
//                 Move::Depart(b) => is_depart_gt_depart(a, b, state, model),
//                 Move::None => is_depart_gt_none(a, state, model),
//                 _ => true,
//             },
//             Move::None => match m {
//                 Move::Board(a) => !is_board_gt_none(a, state, model),
//                 Move::Detrain(a) => !is_detrain_gt_none(a, state, model),
//                 Move::Depart(a) => !is_depart_gt_none(a, state, model),
//                 _ => true,
//             },
//             Move::TrainStart(a) => match m {
//                 Move::TrainStart(b) => is_train_start_gt_train_start(a, b, state, model),
//                 _ => true,
//             },
//         }
//     }

//     pub fn to_string(&self, model: &Model) -> String {
//         match self {
//             Move::Board((t_id, p_id, s_id)) => {
//                 format!(
//                     "Board {} to {} from {}",
//                     model.passengers[*p_id].name,
//                     model.trains[*t_id].name,
//                     model.stations[*s_id].name,
//                 )
//             }
//             Move::Detrain((t_id, p_id, s_id)) => {
//                 format!(
//                     "Detrain {} from {} to {}",
//                     model.passengers[*p_id].name,
//                     model.trains[*t_id].name,
//                     model.stations[*s_id].name,
//                 )
//             }
//             Move::Depart((t_id, (from, to), c_id)) => {
//                 format!(
//                     "Depart {} from {} to {} via {}",
//                     model.trains[*t_id].name,
//                     model.stations[*from].name,
//                     model.stations[*to].name,
//                     model.connections[*c_id].name,
//                 )
//             }
//             Move::TrainStart((t_id, s_id)) => {
//                 format!(
//                     "Start {} on {}",
//                     model.trains[*t_id].name, model.stations[*s_id].name,
//                 )
//             }
//             _ => "".to_string(),
//         }
//     }
// }
