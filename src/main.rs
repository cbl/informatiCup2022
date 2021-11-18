use rstrain::{annealer, connection, cost, entities, passenger, station, timetable, train};

fn main() {
    let s1 = station::Station {
        name: "S1",
        capacity: 5,
    };
    let s2 = station::Station {
        name: "S2",
        capacity: 5,
    };
    let stations: Vec<station::Station> = vec![s1, s2];

    let d1 = connection::Connection {
        name: "L1",
        distance: 1.0,
        capacity: 3,
    };

    let s1_id: usize = 0;
    let s2_id: usize = 1;

    let mut connections: connection::Connections = connection::Connections::new();
    connections.insert((s1_id, s2_id), d1);
    connections.insert((s2_id, s1_id), d1);

    let t1 = train::Train {
        name: "T1",
        start: train::StartStation::Station(0),
        speed: 1.0,
        capacity: 1,
    };
    let t2 = train::Train {
        name: "T2",
        start: train::StartStation::Station(0),
        speed: 1.0,
        capacity: 1,
    };
    // let t3 = train::Train {
    //     name: "T3",
    //     start: train::StartStation::Station(0),
    //     speed: 1.0,
    //     capacity: 1,
    // };
    let trains = vec![t1, t2];

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
    // let p3 = passenger::Passenger {
    //     name: "P3",
    //     start: 0,
    //     destination: 1,
    //     size: 1,
    //     arrival: 4,
    // };
    let passengers = vec![p1, p2];

    let entities: entities::Entities = entities::Entities {
        stations,
        connections,
        trains,
        passengers,
    };

    let mut timetable = timetable::Timetable::new(entities.clone(), entities.init_solution());

    let annealer = annealer::Annealer {};
    annealer.anneal(&mut timetable);

    // timetable.board(0, 0, 1);
    // timetable.depart(0, (0, 1), 2);
    // timetable.depart_random(1);
    // timetable.depart_random(1);
    // timetable.depart_random(2);
    // timetable.depart_random(2);
    // timetable.depart_random(2);
    // timetable.depart_random(2);
    // timetable.depart_random(3);
    // timetable.depart_random(3);
    // timetable.depart_random(4);
    // timetable.depart_random(4);
    // timetable.board_random(1);
    // timetable.board_random(1);
    // timetable.detrain_random(4);
    // timetable.detrain_random(4);

    println!("{}", timetable.to_string());

    println!("cost {}", cost::cost(&timetable));
}
