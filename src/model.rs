use crate::connection::{Connection, Connections, Distance, Id as CId};
use crate::passenger::{Location as PLocation, Passenger};
use crate::rule::Rule;
use crate::rules::get_rules;
use crate::state::State;
use crate::station::{Id as SId, Station};
use crate::train::{Id as TId, Location as TLocation, Speed, StartStation, Train};
use crate::types::{BuildHasher, Capacity, Time, TimeDiff};
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

use rust_decimal::Decimal;

/// Holds a path between two stations and the total distance of the path.
#[derive(Clone)]
pub struct Path {
    /// A vector of all stations building the path.
    pub path: Vec<SId>,

    /// The total distance of the path.
    pub distance: Distance,
}

#[derive(Clone, PartialEq, Debug)]
pub struct ClosestStation {
    pub distance: Distance,
    pub s_id: SId,
}

impl PartialOrd for ClosestStation {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.distance.partial_cmp(&self.distance)
    }
}

impl Eq for ClosestStation {}

impl Ord for ClosestStation {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(&other).unwrap()
    }
}

/// The HashMap type of pairs of stations and the corresponding Path.
type Paths = HashMap<(SId, SId), Path, BuildHasher>;

/// The model struct holds all existing entities and the corresponding meta
/// data. This includes a list of stations, connections, trains and passengers.
pub struct Model {
    /// A vector containing all stations of the model.
    pub stations: Vec<Station>,

    /// A vector containting all connections of the model.
    pub connections: Connections,

    /// A vector containing all trains of the model.
    pub trains: Vec<Train>,

    /// A vector containing all passengers of the model.
    pub passengers: Vec<Passenger>,

    /// Nested vectors mapping all stations ids to the corresponding connection
    /// ids.
    pub station_connections: Vec<Vec<CId>>,

    /// A vector containing all stations ordered by the distance for each station.
    pub closest_stations: Vec<BinaryHeap<ClosestStation>>,

    /// A HashMap mapping pairs of station ids to the corresponding shortest
    /// path bewteen the stations.
    pub paths: Paths,

    /// The latest arrival time of all passengers.
    pub max_arrival: Time,

    /// t max.
    pub t_max: Time,

    /// A vector containing all rules.
    pub rules: Vec<Rule>,

    /// The number of trains used to bring all passengers to the destinations.
    /// The number can be reduced when the amount of trains being used is
    /// slowing down the process of finding the best solution as the trains are
    /// blocking each other.
    pub used_trains: usize,
}

impl Model {
    /// Create new Model instance
    pub fn new(
        stations: Vec<Station>,
        connections: Connections,
        trains: Vec<Train>,
        passengers: Vec<Passenger>,
        rules: Vec<Rule>,
    ) -> Model {
        let max_arrival = passengers.iter().map(|p| p.arrival).max().unwrap();
        let t_len = trains.len();

        let mut station_connections: Vec<Vec<CId>> = stations.iter().map(|_| vec![]).collect();

        for (c_id, connection) in connections.clone().into_iter().enumerate() {
            station_connections[connection.a].push(c_id);
            station_connections[connection.b].push(c_id);
        }

        let paths = shortest_paths(&stations, &connections);

        let closest_stations = (0..stations.len())
            .map(|a| {
                let mut s = BinaryHeap::new();

                for b in 0..stations.len() {
                    let d = paths.get(&(a, b)).unwrap().distance;
                    // let r = Reverse(d);
                    s.push(ClosestStation {
                        s_id: b,
                        distance: d,
                    });
                }

                s
            })
            .collect();

        Model {
            stations,
            connections,
            trains,
            passengers,
            station_connections,
            closest_stations,
            paths,
            max_arrival,
            t_max: max_arrival * 3,
            rules,
            used_trains: t_len,
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
                speed: Speed::ONE,
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
                distance: Distance::TWO,
                capacity: 3,
                a: i,
                b: i + 1,
            })
            .collect();

        Model::new(stations, connections, trains, passengers, get_rules())
    }

    /// The time a train needs to arrive at the given station.
    pub fn train_arrival(&self, t_id: TId, c_id: CId) -> Decimal {
        (self.connections[c_id].distance / self.trains[t_id].speed).ceil()
    }

    /// Gets the destination station for the given start station id and the
    /// connection id.
    pub fn get_destination(&self, s: SId, c: CId) -> SId {
        if self.connections[c].a == s {
            self.connections[c].b
        } else {
            self.connections[c].a
        }
    }

    /// Gets the distance between to stations.
    pub fn distance(&self, a: SId, b: SId) -> Distance {
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
            .map(|_| self.t_max as TimeDiff)
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

fn shortest_paths(stations: &Vec<Station>, connections: &Connections) -> Paths {
    let mut paths = Paths::default();
    let mut distances: Vec<Vec<Distance>> = (0..stations.len())
        .map(|_| (0..stations.len()).map(|_| Distance::ZERO).collect())
        .collect();

    for c in connections {
        distances[c.a][c.b] = c.distance;
        distances[c.b][c.a] = c.distance;
    }

    // for k in 0..stations.len() {
    //     for i in 0..stations.len() {
    //         for j in 0..stations.len() {}
    //     }
    // }

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

        distance[start] = Distance::ZERO;

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
        }
    }

    for s_id in 0..stations.len() {
        paths.insert(
            (s_id, s_id),
            Path {
                path: vec![],
                distance: Distance::ZERO,
            },
        );
    }

    paths
}
