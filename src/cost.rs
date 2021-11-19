use crate::passenger::Location as PLocation;
use crate::passenger::Passenger;
use crate::state::State;
use crate::timetable::Timetable;
use crate::types::Time;

pub fn cost(tt: &Timetable) -> f64 {
    let mut cost = 0.0;
    let mut t_len = tt.solution.0.len();

    cost += tt.solution.0[0]
        .t_location
        .clone()
        .into_iter()
        .filter(|l| l.is_nothing())
        .count() as f64
        * 10.0;

    tt.solution.0.iter().enumerate().for_each(|(t, state)| {
        let a = arrival_cost(&tt.entities.passengers, &state, t, |a_i, a_s| {
            match a_s == a_i {
                true => 1.0,
                false => 1.0 / (a_s - a_i),
            }
        });

        let t_s_w = train_s_w_cost(&state, t, |w| w);
        let t_c_w = train_c_w_cost(&state, t, |w| w);
        let p_s_w = passenger_s_w_cost(&state, t, |p_id, w| {
            w //* (tt.entities.passengers[p_id].arrival / t_len + 1) as f64
        });
        let p_t_w = passenger_t_w_cost(&state, t, |p_id, w| {
            w //* */ (tt.entities.passengers[p_id].arrival / t_len + 1) as f64
        });

        cost += a
            + 1.0 / (t_s_w + 1.0)
            + (1.0 / (t_c_w + 1.0))
            + (1.0 / (p_s_w + 1.0))
            + (2.0 / (p_t_w + 1.0));
    });

    let time_per_entity = tt.solution.0.len() as f64
        / (tt.entities.trains.len() as f64 + tt.entities.passengers.len() as f64);

    cost / time_per_entity
}

fn train_s_w_cost<F>(state: &State, t: Time, f: F) -> f64
where
    F: Fn(f64) -> f64,
{
    state
        .t_location
        .iter()
        .filter(|l| l.is_station())
        .map(|_| f(t as f64))
        .sum()
}

fn train_c_w_cost<F>(state: &State, t: Time, f: F) -> f64
where
    F: Fn(f64) -> f64,
{
    state
        .t_location
        .iter()
        .filter(|l| l.is_connection())
        .map(|_| f(t as f64))
        .sum()
}

fn passenger_s_w_cost<F>(state: &State, t: Time, f: F) -> f64
where
    F: Fn(usize, f64) -> f64,
{
    state
        .p_location
        .iter()
        .filter(|l| l.is_station())
        .enumerate()
        .map(|(p_id, _)| f(p_id, t as f64))
        .sum()
}
fn passenger_t_w_cost<F>(state: &State, t: Time, f: F) -> f64
where
    F: Fn(usize, f64) -> f64,
{
    state
        .p_location
        .iter()
        .filter(|l| l.is_train())
        .enumerate()
        .map(|(p_id, _)| f(p_id, t as f64))
        .sum()
}

fn arrival_cost<F>(passengers: &Vec<Passenger>, state: &State, t: Time, f: F) -> f64
where
    F: Fn(f64, f64) -> f64,
{
    state
        .p_location
        .iter()
        .enumerate()
        .filter(|(p_id, l)| l.is_arrived())
        .map(|(p_id, l)| f(t as f64, passengers[p_id].arrival as f64))
        .sum()
}

pub fn delays(tt: &Timetable) -> i32 {
    tt.solution
        .0
        .clone()
        .into_iter()
        .enumerate()
        .map(|(t, s)| {
            s.p_location
                .into_iter()
                .filter(|l| l.is_arrived())
                .enumerate()
                .filter(|(p_id, l)| tt.entities.passengers[*p_id].arrival < t)
                .count() as i32
        })
        .sum()
}
