use crate::connection::{Connection, Connections, Distance};
use crate::model::Model;
use crate::passenger::Passenger;
use crate::station::Station;
use crate::train::{Speed, StartStation, Train};
use crate::types;
use regex::Regex;
use std::collections::HashMap;

pub fn parse(string: &String) -> Model {
    let re_block = Regex::new(r"(?s)(\[(.*?)\].*?)(?:(?:\r*\n){2})").unwrap();
    let re_entity = Regex::new(r"(^|\s)+([^\s]+)").unwrap();

    let mut stations: Vec<Station> = vec![];
    let mut trains: Vec<Train> = vec![];
    let mut passengers: Vec<Passenger> = vec![];
    let mut connections: Connections = Connections::new();
    let mut station_ids: HashMap<String, types::Id> = HashMap::new();

    for result in re_block.captures_iter(&(string.clone() + "\n\n")) {
        assert_eq!(result.len(), 3, "Parsing failed!");
        match &result[2] {
            "Stations" => {
                result[0]
                    .lines()
                    .skip(1)
                    .filter(|s| *s != "")
                    .for_each(|line| {
                        let data: Vec<String> = re_entity
                            .find_iter(line)
                            .filter_map(|digits| digits.as_str().parse().ok())
                            .collect::<Vec<String>>()
                            .into_iter()
                            .map(|s| s.replace(" ", ""))
                            .collect();

                        assert_eq!(data.len(), 2, "Failed to read station \"{}\"!", line);

                        let station = Station {
                            name: data[0].to_string(),
                            capacity: data[1].parse::<types::Capacity>().unwrap(),
                        };

                        station_ids.insert(station.name.clone(), stations.len());
                        stations.push(station);
                    });
            }
            "Lines" => {
                result[0]
                    .lines()
                    .skip(1)
                    .filter(|s| *s != "")
                    .for_each(|line| {
                        let data: Vec<String> = re_entity
                            .find_iter(line)
                            .filter_map(|digits| digits.as_str().parse().ok())
                            .collect::<Vec<String>>()
                            .into_iter()
                            .map(|s| s.replace(" ", ""))
                            .collect();

                        assert_eq!(data.len(), 5, "Failed to read line \"{}\"!", line);

                        let from = station_ids[&data[1]];
                        let to = station_ids[&data[2]];

                        let connection = Connection {
                            name: data[0].to_string(),
                            distance: data[3].parse::<Distance>().unwrap(),
                            capacity: data[4].parse::<types::Capacity>().unwrap(),
                        };

                        connections.insert((from, to), connection.clone());
                        connections.insert((to, from), connection);
                    });
            }
            "Trains" => {
                result[0]
                    .lines()
                    .skip(1)
                    .filter(|s| *s != "")
                    .for_each(|line| {
                        let data: Vec<String> = re_entity
                            .find_iter(line)
                            .filter_map(|digits| digits.as_str().parse().ok())
                            .collect::<Vec<String>>()
                            .into_iter()
                            .map(|s| s.replace(" ", ""))
                            .collect();

                        assert_eq!(data.len(), 4, "Failed to read train \"{}\"!", line);

                        let start = if data[1] == "*" {
                            StartStation::Any
                        } else {
                            StartStation::Station(station_ids[&data[1]])
                        };

                        let train = Train {
                            name: data[0].to_string(),
                            start,
                            speed: data[2].parse::<Speed>().unwrap(),
                            capacity: data[3].parse::<types::Capacity>().unwrap(),
                        };

                        trains.push(train);
                    });
            }
            "Passengers" => {
                result[0]
                    .lines()
                    .skip(1)
                    .filter(|s| *s != "")
                    .for_each(|line| {
                        let data: Vec<String> = re_entity
                            .find_iter(line)
                            .filter_map(|digits| digits.as_str().parse().ok())
                            .collect::<Vec<String>>()
                            .into_iter()
                            .map(|s| s.replace(" ", ""))
                            .collect();

                        assert_eq!(data.len(), 5, "Failed to read passenger \"{}\"!", line);

                        let passenger = Passenger {
                            name: data[0].to_string(),
                            start: station_ids[&data[1]],
                            destination: station_ids[&data[2]],
                            size: data[3].parse::<i32>().unwrap(),
                            arrival: data[4].parse::<usize>().unwrap(),
                        };

                        passengers.push(passenger);
                    });
            }
            _ => {}
        };
    }

    Model::new(stations, connections, trains, passengers)
}
