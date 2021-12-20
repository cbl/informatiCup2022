use crate::connection::{Connection, Connections, Distance, Id as CId};
use crate::passenger::{Id as PId, Location as PLocation, Passenger};
use crate::state::State;
use crate::station::{Id as SId, Station};
use crate::train::{Location as TLocation, StartStation, Train};
use crate::types::{Capacity, Fitness, Time};

use std::collections::HashMap;

#[derive(Clone)]
pub struct Path {
    path: Vec<SId>,
    distance: Distance,
}

/// The model struct holds all existing entities and the corresponding meta
/// data. This includes a list of stations, connections, trains and passengers.
#[derive(Clone)]
pub struct Model {
    pub stations: Vec<Station>,
    pub connections: Connections,
    pub trains: Vec<Train>,
    pub passengers: Vec<Passenger>,
    pub station_connections: Vec<Vec<CId>>,
    pub paths: HashMap<(SId, SId), Path>,
    pub max_distance: Distance,
    pub max_arrival: Time,
    pub t_max: Time,
}

impl Model {
    /// Create new Model instance
    pub fn new(
        stations: Vec<Station>,
        connections: Connections,
        trains: Vec<Train>,
        passengers: Vec<Passenger>,
    ) -> Model {
        let mut max_arrival = 0;

        for p in &passengers {
            if p.arrival > max_arrival {
                max_arrival = p.arrival;
            }
        }

        let mut station_connections: Vec<Vec<CId>> = stations.iter().map(|_| vec![]).collect();

        for (c_id, connection) in connections.clone().into_iter().enumerate() {
            station_connections[connection.a].push(c_id);
            station_connections[connection.b].push(c_id);
        }

        let (paths, max_distance) = shortest_paths(&stations, &connections);

        Model {
            stations,
            connections,
            trains,
            passengers,
            station_connections,
            paths,
            max_distance,
            max_arrival,
            t_max: max_arrival,
        }
    }

    pub fn new_for_bench() -> Model {
        let stations = (0..10)
            .map(|i| Station {
                name: format!("S{}", i),
                capacity: 3,
            })
            .collect();

        let trains = (0..10)
            .map(|i| Train {
                name: format!("T{}", i),
                start: StartStation::Station(i),
                capacity: 10,
                speed: 1.0,
            })
            .collect();

        let passengers = (0..9)
            .map(|i| Passenger {
                name: format!("P{}", i),
                start: i,
                destination: i + 1,
                size: 2,
                arrival: 10,
            })
            .collect();

        let connections = (0..9)
            .map(|i| Connection {
                name: format!("L{}", i),
                distance: 2.0,
                capacity: 3,
                a: i,
                b: i + 1,
            })
            .collect();

        Model::new(stations, connections, trains, passengers)
    }

    pub fn normalize_distance(&self, d: Distance) -> Fitness {
        d as Fitness / self.max_distance as Fitness
    }

    pub fn normalized_arrival(&self, p_id: PId) -> Fitness {
        self.passengers[p_id].arrival as Fitness / self.max_arrival as Fitness
    }

    pub fn get_destination(&self, s: SId, c: CId) -> SId {
        if self.connections[c].a == s {
            return self.connections[c].b;
        }
        self.connections[c].a
    }

    pub fn distance(&self, a: SId, b: SId) -> f64 {
        self.paths.get(&(a, b)).unwrap().distance
    }

    /// Gets the initial state from the model.
    pub fn initial_state(&self) -> State {
        let mut s_capacity: Vec<Capacity> = self
            .stations
            .clone()
            .into_iter()
            .map(|station| station.capacity)
            .collect::<Vec<Capacity>>();

        let t_capacity = self
            .trains
            .clone()
            .into_iter()
            .map(|train| {
                if let StartStation::Station(s_id) = train.start {
                    s_capacity[s_id] -= 1;
                }

                train.capacity
            })
            .collect::<Vec<Capacity>>();

        let t_location = self
            .trains
            .clone()
            .into_iter()
            .map(|train| train.start.to_location())
            .collect::<Vec<TLocation>>();

        let p_location = self
            .passengers
            .clone()
            .into_iter()
            .map(|passenger| PLocation::Station(passenger.start))
            .collect::<Vec<PLocation>>();

        let c_capacity = self
            .connections
            .clone()
            .into_iter()
            .map(|connection| connection.capacity)
            .collect::<Vec<Capacity>>();

        let p_delays = (0..self.passengers.len())
            .map(|_| self.t_max as i32)
            .collect();

        State::new(
            0,
            s_capacity.clone(),
            c_capacity.clone(),
            t_capacity.clone(),
            t_location.clone(),
            p_location.clone(),
            p_delays,
        )
    }
}

fn shortest_paths(
    stations: &Vec<Station>,
    connections: &Connections,
) -> (HashMap<(SId, SId), Path>, Distance) {
    let mut paths = HashMap::new();
    let mut max_distance: Distance = Distance::MIN;
    let mut distances: Vec<Vec<Distance>> = (0..stations.len())
        .map(|_| (0..stations.len()).map(|_| 0.0).collect())
        .collect();

    for c in connections {
        distances[c.a][c.b] = c.distance;
        distances[c.b][c.a] = c.distance;
    }

    let edge_list: Vec<Vec<SId>> = (0..stations.len())
        .map(|s_id| {
            connections
                .iter()
                .filter_map(|connection| {
                    if connection.a == s_id {
                        return Some(connection.b);
                    } else if connection.b == s_id {
                        return Some(connection.a);
                    }

                    None
                })
                .collect()
        })
        .collect();

    for start in 0..(stations.len() - 1) {
        let mut edges;
        let mut in_tree = vec![];
        let mut parent = vec![];
        let mut distance = vec![];
        let mut v: SId = start; // current vertex to process
        let mut dist; // best current distance from start
        let mut weight;

        for _ in 0..stations.len() {
            in_tree.push(false);
            distance.push(Distance::MAX);
            parent.push(None);
        }

        distance[start] = 0.0;

        while !in_tree[v] {
            in_tree[v] = true;
            edges = edge_list[v].clone();

            while let Some(w) = edges.pop() {
                weight = distances[v][w];
                if distance[w] > distance[v] + weight {
                    distance[w] = distance[v] + weight;
                    parent[w] = Some(v);
                }
            }

            v = 1;
            dist = Distance::MAX;
            for i in 0..stations.len() {
                if !in_tree[i] && dist > distance[i] {
                    dist = distance[i];
                    v = i;
                }
            }
        }

        for destination in (start + 1)..stations.len() {
            let mut path = vec![destination];

            v = destination;

            while let Some(next) = parent[v] {
                v = next;
                path.push(v);
            }

            paths.insert(
                (destination, start),
                Path {
                    path: path.clone(),
                    distance: distance[destination],
                },
            );
            paths.insert(
                (start, destination),
                Path {
                    path: path.clone().into_iter().rev().collect(),
                    distance: distance[destination],
                },
            );

            if distance[destination] > max_distance {
                max_distance = distance[destination];
            }
        }
    }

    for s_id in 0..stations.len() {
        paths.insert(
            (s_id, s_id),
            Path {
                path: vec![],
                distance: 0.0,
            },
        );
    }

    (paths, max_distance)
}
