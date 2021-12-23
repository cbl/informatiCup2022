use crate::rule::{Closure, ClosureAny, Result, Rule};

/// - Depart trains from the station when it will be overloaded at t+1.
/// - Do not depart trains towards stations that will be overloaded at arrival.
pub fn rules() -> Vec<Rule> {
    vec![
        // depart vs none
        // Rule::IsDepartGtNone(Closure {
        //     c: Box::new(|a, _, state, model| {
        //         let t_arrival = model.train_arrival(a.t_id, a.c_id);

        //         let from_est_cap = state.est_s_cap(state.t + t_arrival, a.from, model) - 1; // -1 because the train will leave
        //         let from_est_trains = model.stations[a.from].capacity - from_est_cap;

        //         let to_est_cap = state.est_s_cap(state.t + t_arrival, a.to, model) + 1; // +1 because the train will arrive
        //         let to_est_trains = model.stations[a.to].capacity - to_est_cap;

        //         let from_cap: i16 = model.station_connections[a.from]
        //             .iter()
        //             .map(|c_id| {
        //                 let mut capacity = state.c_capacity[*c_id];

        //                 if *c_id == a.c_id {
        //                     capacity -= 1;
        //                 }

        //                 capacity
        //             })
        //             .sum();

        //         let to_cap: i16 = model.station_connections[a.to]
        //             .iter()
        //             .map(|c_id| {
        //                 let mut capacity = state.c_capacity[*c_id];

        //                 if *c_id == a.c_id {
        //                     capacity -= 1;
        //                 }

        //                 capacity
        //             })
        //             .sum();
        //         if from_cap < from_est_trains || to_cap < to_est_trains {
        //             // return Result::Some(false);
        //         }
        //         // for c_id in model.station_connections[s_id]

        //         // if state.est_s_cap(state.t + model.train_arrival(a.t_id, a.c_id), a.to, model)
        //         //     <= (-model.stations[a.to].capacity + 1)
        //         // if to_est_cap <= 0 {
        //         //     // if state.s_capacity[a.to] <= 1 {
        //         //     return Result::Some(false);
        //         // }

        //         // Result::Some(false)
        //         Result::None
        //     }),
        // }),
        Rule::IsDepartGtAny(ClosureAny {
            c: Box::new(|a, state, model| {
                if state.est_s_cap(state.t + 1, a.from, model) <= 0 {
                    // println!("{}", a.to_string(model));
                    Result::Some(true)
                } else {
                    Result::None
                }
            }),
        }),
        Rule::IsDepartGtNone(Closure {
            c: Box::new(|a, _, state, model| {
                let est_s_cap = state.est_s_cap(model.t_max, a.to, model);
                let s_cap = model.stations[a.to].capacity;
                let s_blockers =
                    state.get_blockers(model.train_arrival(a.t_id, a.c_id), a.to, model);

                let s_c_cap: i16 = model.station_connections[a.to]
                    .iter()
                    .map(|c_id| state.c_capacity[*c_id])
                    .sum();

                if (est_s_cap - 2) <= -s_cap || s_c_cap - 1 < 1 || s_blockers >= s_cap {
                    Result::Some(false)
                } else {
                    Result::None
                }
            }),
        }),
        // Rule::IsBoardGtNone(Closure {
        //     c: Box::new(|a, _, state, model| {
        //         if state.est_s_cap(state.t + 2, a.s_id, model) < 0 {
        //             Result::Some(false)
        //         } else {
        //             Result::None
        //         }
        //     }),
        // }),
        Rule::IsStartGtNone(Closure {
            c: Box::new(|a, _, state, model| {
                if state.est_s_cap(model.t_max, a.s_id, model) <= 0 {
                    Result::Some(false)
                } else {
                    Result::None
                }
            }),
        }),
    ]
}
