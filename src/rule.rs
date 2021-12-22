use crate::model::Model;
use crate::move_::{Board, Depart, Detrain, None, Start};
use crate::move_::{Move, MoveStruct};
use crate::state::State;
use std::ops::Not;

pub enum Result {
    Some(bool),
    None,
}

impl Result {
    pub fn is_some(&self) -> bool {
        match self {
            Result::None => false,
            _ => true,
        }
    }

    pub fn is_none(&self) -> bool {
        !self.is_some()
    }
}

impl Not for Result {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Result::Some(r) => Result::Some(!r),
            Result::None => Result::None,
        }
    }
}

pub struct Closure<A, B>
where
    A: MoveStruct,
    B: MoveStruct,
{
    c: Box<dyn Fn(&A, &B) -> Result>,
}

pub enum Rule {
    IsBoardGtBoard(Closure<Board, Board>),
    IsDetrainGtDetrain(Closure<Detrain, Detrain>),
    IsDepartGtDepart(Closure<Depart, Depart>),
    IsStartGtStart(Closure<Start, Start>),

    IsBoardGtDetrain(Closure<Board, Detrain>),
    IsBoardGtDepart(Closure<Board, Depart>),
    IsBoardGtStart(Closure<Board, Start>),
    IsBoardGtNone(Closure<Board, None>),

    IsDetrainGtDepart(Closure<Detrain, Depart>),
    IsDetrainGtStart(Closure<Detrain, Start>),
    IsDetrainGtNone(Closure<Detrain, None>),

    IsDepartGtStart(Closure<Depart, Start>),
    IsDepartGtNone(Closure<Depart, None>),

    IsStartGtNone(Closure<Start, None>),
}

impl Rule {
    pub fn cmp(&self, a: &Move, b: &Move) -> Result {
        // board vs board
        if let (Rule::IsBoardGtBoard(rule), Move::Board(a), Move::Board(b)) = (self, a, b) {
            return (rule.c)(a, b);
        }
        // detrain vs detrain
        if let (Rule::IsDetrainGtDetrain(rule), Move::Detrain(a), Move::Detrain(b)) = (self, a, b) {
            return (rule.c)(a, b);
        }
        // depart vs depart
        if let (Rule::IsDepartGtDepart(rule), Move::Depart(a), Move::Depart(b)) = (self, a, b) {
            return (rule.c)(a, b);
        }
        // depart vs depart
        if let (Rule::IsStartGtStart(rule), Move::Start(a), Move::Start(b)) = (self, a, b) {
            return (rule.c)(a, b);
        }

        // board vs detrain
        if let Rule::IsBoardGtDetrain(rule) = self {
            if let (Move::Board(a), Move::Detrain(b)) = (a, b) {
                return (rule.c)(a, b);
            }
            if let (Move::Board(b), Move::Detrain(a)) = (b, a) {
                return !(rule.c)(b, a);
            }
        }
        // board vs depart
        if let Rule::IsBoardGtDepart(rule) = self {
            if let (Move::Board(a), Move::Depart(b)) = (a, b) {
                return (rule.c)(a, b);
            }
            if let (Move::Board(b), Move::Depart(a)) = (b, a) {
                return !(rule.c)(b, a);
            }
        }
        // board vs detrain
        if let Rule::IsBoardGtStart(rule) = self {
            if let (Move::Board(a), Move::Start(b)) = (a, b) {
                return (rule.c)(a, b);
            }
            if let (Move::Board(b), Move::Start(a)) = (b, a) {
                return !(rule.c)(b, a);
            }
        }
        // board vs none
        if let Rule::IsBoardGtNone(rule) = self {
            if let (Move::Board(a), Move::None(b)) = (a, b) {
                return (rule.c)(a, b);
            }
            if let (Move::Board(b), Move::None(a)) = (b, a) {
                return !(rule.c)(b, a);
            }
        }

        // detrain vs depart
        if let Rule::IsDetrainGtDepart(rule) = self {
            if let (Move::Detrain(a), Move::Depart(b)) = (a, b) {
                return (rule.c)(a, b);
            }
            if let (Move::Detrain(b), Move::Depart(a)) = (b, a) {
                return !(rule.c)(b, a);
            }
        }
        // detrain vs start
        if let Rule::IsDetrainGtStart(rule) = self {
            if let (Move::Detrain(a), Move::Start(b)) = (a, b) {
                return (rule.c)(a, b);
            }
            if let (Move::Detrain(b), Move::Start(a)) = (b, a) {
                return !(rule.c)(b, a);
            }
        }
        // detrain vs none
        if let Rule::IsDetrainGtNone(rule) = self {
            if let (Move::Detrain(a), Move::None(b)) = (a, b) {
                return (rule.c)(a, b);
            }
            if let (Move::Detrain(b), Move::None(a)) = (b, a) {
                return !(rule.c)(b, a);
            }
        }

        // depart vs start
        if let Rule::IsDepartGtStart(rule) = self {
            if let (Move::Depart(a), Move::Start(b)) = (a, b) {
                return (rule.c)(a, b);
            }
            if let (Move::Depart(b), Move::Start(a)) = (b, a) {
                return !(rule.c)(b, a);
            }
        }
        // depart vs none
        if let Rule::IsDepartGtNone(rule) = self {
            if let (Move::Depart(a), Move::None(b)) = (a, b) {
                return (rule.c)(a, b);
            }
            if let (Move::Depart(b), Move::None(a)) = (b, a) {
                return !(rule.c)(b, a);
            }
        }

        // start vs none
        if let Rule::IsStartGtNone(rule) = self {
            if let (Move::Start(a), Move::None(b)) = (a, b) {
                return (rule.c)(a, b);
            }
            if let (Move::Start(b), Move::None(a)) = (b, a) {
                return !(rule.c)(b, a);
            }
        }

        Result::None
    }
}
