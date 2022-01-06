use crate::connection::{Connection, Connections, Distance};
use crate::model::Model;
use crate::passenger::Passenger;
use crate::rules::get_rules;
use crate::station::Station;
use crate::train::{Speed, StartStation, Train};
use crate::types;
use regex::Regex;
use std::cmp::Ordering;
use std::collections::HashMap;

pub type StationIds = HashMap<String, types::Id>;

/// Parses an instance of a [Model] from an input string.
pub fn parse(string: &String) -> Model {
    let filtered = filter(&string) + "\n\n";

    let stations: Vec<Station> = parse_stations(&filtered);
    let mut station_ids: StationIds = StationIds::new();
    for (s_id, station) in stations.iter().enumerate() {
        station_ids.insert(station.name.clone(), s_id);
    }

    let mut trains: Vec<Train> = parse_trains(&filtered, &station_ids);
    let passengers: Vec<Passenger> = parse_passengers(&filtered, &station_ids);
    let connections: Connections = parse_connections(&filtered, &station_ids);

    // order trains by speed
    trains.sort_by(|a, b| match a.speed > b.speed {
        true => Ordering::Less,
        false => Ordering::Greater,
    });

    Model::new(stations, connections, trains, passengers, get_rules())
}

fn filter(mut string: &str) -> String {
    let re_header = Regex::new(r"\[(.*?)\]").unwrap();
    let lines = string
        .lines()
        .filter(|line| *line != "")
        .filter(|line| match line.chars().next() {
            Some('#') => false,
            _ => true,
        })
        .map(|line| line.to_string())
        .collect::<Vec<String>>();

    let mut result = String::from("");

    for (i, line) in lines.into_iter().enumerate() {
        if re_header.is_match(&line) {
            result += "\n";
        }
        result += &line;
        result += "\n";
    }

    result
}

fn parse_lines(string: &String, name: &str) -> Vec<String> {
    let re_block = Regex::new(r"(?s)(\[(.*?)\].*?)(?:(?:\r*\n){2})").unwrap();
    re_block
        .captures_iter(&(filter(&string) + "\n\n"))
        .filter(|result| &result[2] == name)
        .map(|result| {
            result[0]
                .lines()
                .skip(1)
                .filter(|line| *line != "")
                .map(|line| line.to_string())
                .collect::<Vec<String>>()
        })
        .flatten()
        .collect()
}

fn parse_attributes(line: &String) -> Vec<String> {
    Regex::new(r"(^|\s)+([^\s]+)")
        .unwrap()
        .find_iter(line)
        .filter_map(|digits| digits.as_str().parse().ok())
        .collect::<Vec<String>>()
        .into_iter()
        .map(|s| s.replace(" ", ""))
        .collect()
}

fn parse_stations(string: &String) -> Vec<Station> {
    parse_lines(string, "Stations")
        .iter_mut()
        .map(|line| {
            let attributes = parse_attributes(line);

            assert_eq!(attributes.len(), 2, "Failed to read station \"{}\"!", line);

            Station {
                name: attributes[0].to_string(),
                capacity: attributes[1].parse::<types::Capacity>().unwrap(),
            }
        })
        .collect()
}

fn parse_trains(string: &String, station_ids: &StationIds) -> Vec<Train> {
    parse_lines(string, "Trains")
        .iter_mut()
        .map(|line| {
            let attributes = parse_attributes(line);

            assert_eq!(attributes.len(), 4, "Failed to read train \"{}\"!", line);

            let start = if attributes[1] == "*" {
                StartStation::Any
            } else {
                StartStation::Station(station_ids[&attributes[1]])
            };

            Train {
                name: attributes[0].to_string(),
                start,
                speed: attributes[2].parse::<Speed>().unwrap(),
                capacity: attributes[3].parse::<types::Capacity>().unwrap(),
            }
        })
        .collect()
}

fn parse_passengers(string: &String, station_ids: &StationIds) -> Vec<Passenger> {
    parse_lines(string, "Passengers")
        .iter_mut()
        .map(|line| {
            let attributes = parse_attributes(line);

            assert_eq!(
                attributes.len(),
                5,
                "Failed to read passenger \"{}\"!",
                line
            );

            Passenger {
                name: attributes[0].to_string(),
                start: station_ids[&attributes[1]],
                destination: station_ids[&attributes[2]],
                size: attributes[3].parse::<types::Capacity>().unwrap(),
                arrival: attributes[4].parse::<usize>().unwrap(),
            }
        })
        .collect()
}

fn parse_connections(string: &String, station_ids: &StationIds) -> Vec<Connection> {
    parse_lines(string, "Lines")
        .iter_mut()
        .map(|line| {
            let attributes = parse_attributes(line);

            assert_eq!(attributes.len(), 5, "Failed to read line \"{}\"!", line);

            let from = station_ids[&attributes[1]];
            let to = station_ids[&attributes[2]];

            Connection {
                name: attributes[0].to_string(),
                distance: attributes[3].parse::<Distance>().unwrap(),
                capacity: attributes[4].parse::<types::Capacity>().unwrap(),
                a: from,
                b: to,
            }
        })
        .collect()
}

#[test]
fn it_parses_multiple_sets_of_entities() {
    let string = "
[Stations]
A 2
B 2
[Lines]
L1 A B 3.14 1
L2 A C 3.14 1
[Trains]
Trains * 1 1
[Stations]
C 2
[Passengers]
P1 A B 10 3    
P1 C B 10 3    
";
    let model = parse(&string.to_owned());

    assert_eq!(model.stations.len(), 3);
}
