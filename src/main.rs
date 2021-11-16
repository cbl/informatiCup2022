use rstrain::{connection, entities, passenger, state, station, timetable, train};

fn main() {
    let s1 = station::Station {
        name: "S1",
        capacity: 5,
    };
    let s2 = station::Station {
        name: "S2",
        capacity: 5,
    };
    let stations = vec![s1, s2];

    let connections: connection::Connections = vec![vec![0, 3], vec![3, 0]];

    let t1 = train::Train {
        name: "T1",
        start: 0,
        speed: 1.0,
        capacity: 1,
    };
    let t2 = train::Train {
        name: "T2",
        start: 0,
        speed: 1.0,
        capacity: 1,
    };
    let t3 = train::Train {
        name: "T3",
        start: 0,
        speed: 1.0,
        capacity: 1,
    };
    let trains = vec![t1, t2, t3];

    let p1 = passenger::Passenger {
        name: "P1",
        start: 0,
        destination: 1,
        size: 1,
        arrival: 4,
    };
    let p2 = passenger::Passenger {
        name: "P2",
        start: 0,
        destination: 1,
        size: 1,
        arrival: 4,
    };
    let p3 = passenger::Passenger {
        name: "P3",
        start: 0,
        destination: 1,
        size: 1,
        arrival: 4,
    };
    let passengers = vec![p1];

    let entities = entities::Entities {
        stations,
        connections,
        trains,
        passengers,
    };

    let timetable = timetable::Timetable {
        entities,
        states: entities.init_states(),
    };

    println!("{}", timetable.to_string());
}