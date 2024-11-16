use itertools::Itertools;
use std::collections::HashSet;
use std::num::ParseIntError;

#[derive(Debug, PartialEq)]
struct Route {
    origin: String,
    destination: String,
    distance: u32,
}

#[derive(Debug, PartialEq)]
enum ParseError {
    #[allow(dead_code)]
    InvalidInput,
    ParseIntError(ParseIntError),
}

fn parse(line: &str) -> Result<Route, ParseError> {
    let parts: Vec<&str> = line.split(" = ").collect();
    let locations: Vec<&str> = parts[0].split(" to ").collect();

    parts[1]
        .parse()
        .map_err(ParseError::ParseIntError)
        .map(|distance| Route {
            origin: locations[0].to_string(),
            destination: locations[1].to_string(),
            distance,
        })
}

pub fn part_1(input: &str) -> i32 {
    let routes: Vec<Route> = input
        .lines()
        .map(parse)
        .collect::<Result<Vec<_>, _>>()
        .expect("Failed to parse input");

    let places: HashSet<_> = routes
        .iter()
        .flat_map(|r| vec![&r.origin, &r.destination])
        .collect();

    let mut min_distance = u32::MAX;

    for permutation in places.iter().permutations(places.len()) {
        let mut total_distance = 0;
        for window in permutation.windows(2) {
            if let Some(route) = routes.iter().find(|r| {
                (r.origin == **window[0] && r.destination == **window[1])
                    || (r.origin == **window[1] && r.destination == **window[0])
            }) {
                total_distance += route.distance;
            } else {
                total_distance = u32::MAX;
                break;
            }
        }
        if total_distance < min_distance {
            min_distance = total_distance;
        }
    }

    min_distance as i32
}

pub fn part_2(_input: &str) -> i32 {
    panic!("Not yet implemented");
}

generate_tests!(
    2015,           // year
    9,              // day
    part_1,         // part 1 function
    part_2,         // part 2 function
    vec![("London to Dublin = 464\nLondon to Belfast = 518\nDublin to Belfast = 141", 605)], // part 1 examples
    vec![], // part 2 examples
    Some(207),             // part 1 expected
    None              // part 2 expected
);

#[cfg(test)]
mod local_tests {
    use super::*;
    use test_case::test_case;

    #[test_case(
        "London to Dublin = 464",
        Ok(Route {
            origin: "London".to_string(),
            destination: "Dublin".to_string(),
            distance: 464
        });
        "parse line 1"
    )]
    #[test_case(
        "London to Belfast = 518",
        Ok(Route {
            origin: "London".to_string(),
            destination: "Belfast".to_string(),
            distance: 518
        });
        "parse line 2"
    )]
    #[test_case(
        "Dublin to Belfast = 141",
        Ok(Route {
            origin: "Dublin".to_string(),
            destination: "Belfast".to_string(),
            distance: 141
        });
        "parse line 3"
    )]
    fn parse_works(
        input: &str,
        expected: Result<Route, ParseError>,
    ) {
        assert_eq!(parse(input), expected);
    }
}
