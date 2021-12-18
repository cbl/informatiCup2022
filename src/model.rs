use crate::connection::{Connections, Distance, Id as CId, Name as CName};
use crate::move_::Move;
use crate::passenger::{ArrivalTime as PArrivalTime, Location as PLocation, Passenger};
use crate::state::State;
use crate::station::{Id as SId, Station};
use crate::train::{Location as TLocation, Train};
use crate::types::{Capacity, Time};

use std::collections::{BinaryHeap, HashMap};

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
    pub paths: HashMap<CId, Path>,
    pub max_distance: Distance,
}

impl Model {
    /// Create new Model instance
    pub fn new(
        stations: Vec<Station>,
        connections: Connections,
        trains: Vec<Train>,
        passengers: Vec<Passenger>,
    ) -> Model {
        let (paths, max_distance) = shortest_paths(&stations, &connections);

        Model {
            stations,
            connections,
            trains,
            passengers,
            paths,
            max_distance,
        }
    }

    /// Gets the latest arrival time of all passengers.
    pub fn latest_arrival(&self) -> PArrivalTime {
        let mut t = 0;

        for p in &self.passengers {
            if p.arrival > t {
                t = p.arrival;
            }
        }

        return t;
    }

    pub fn get_destination(&self, s: SId, c: CId) -> Option<SId> {
        if c.0 == s {
            return Some(c.1);
        } else if c.1 == s {
            return Some(c.0);
        }

        None
    }

    pub fn distance(&self, a: SId, b: SId) -> f64 {
        self.paths.get(&(a, b)).unwrap().distance
    }

    /// Gets the initial state from the model.
    pub fn initial_state(&self) -> State {
        let s_capacity: Vec<Capacity> = self
            .stations
            .clone()
            .into_iter()
            .map(|station| station.capacity)
            .collect::<Vec<Capacity>>();

        let t_capacity = self
            .trains
            .clone()
            .into_iter()
            .map(|train| train.capacity)
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
            .map(|(_, connection)| (connection.name, connection.capacity))
            .collect::<HashMap<CName, Capacity>>();

        let p_delays = (0..self.passengers.len())
            .map(|_| self.latest_arrival() as i32)
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
) -> (HashMap<CId, Path>, Distance) {
    let mut paths = HashMap::new();
    let mut max_distance: Distance = Distance::MIN;
    let distances: Vec<Vec<Distance>> = (0..stations.len())
        .map(|a| {
            (0..stations.len())
                .map(|b| {
                    if let Some(c) = connections.get(&(a, b)) {
                        return c.distance;
                    } else {
                        return 0.0;
                    }
                })
                .collect()
        })
        .collect();

    let edge_list: Vec<Vec<SId>> = (0..stations.len())
        .map(|s_id| {
            connections
                .iter()
                .filter_map(|((a, b), _)| if *a == s_id { return Some(*b) } else { None })
                .collect()
        })
        .collect();

    for start in 0..(stations.len() - 1) {
        let mut edges;
        let mut in_tree = vec![];
        let mut parent = vec![];
        let mut distance = vec![];
        let mut v: SId = start; // current vertex to process
        let mut w: SId; // canidate next vertice
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
