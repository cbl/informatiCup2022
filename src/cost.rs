use crate::passenger::Location as PLocation;
use crate::timetable::Timetable;

pub fn cost(tt: &Timetable) -> f64 {
    let mut cost = 0.0;

    for t in 0..tt.solution.0.len() {
        // waiting passengers
        cost += tt.solution.0[t]
            .p_location
            .iter()
            .filter(|location| match location {
                PLocation::Station(_) => true,
                _ => false,
            })
            .count() as f64
            * 2.0;

        // arrived passengers
        cost -= tt.solution.0[t]
            .p_location
            .iter()
            .filter(|location| match location {
                PLocation::Arrived => true,
                _ => false,
            })
            .count() as f64
            * 2.0;

        // traveling passengers
        cost += tt.solution.0[t]
            .p_location
            .iter()
            .filter(|location| match location {
                PLocation::Train(_) => true,
                _ => false,
            })
            .count() as f64
            * 0.8;

        // sleeping trains
        cost += tt.solution.0[t]
            .t_location
            .iter()
            .enumerate()
            .filter(|(t_id, _)| tt.solution.is_train_sleeping(*t_id, t))
            .count() as f64
            * 0.2;
    }

    cost
}
