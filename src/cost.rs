use crate::state;

pub fn cost(state: &state::State, data: &state::Data) -> f64 {
    return state.passenger_delay();
}
