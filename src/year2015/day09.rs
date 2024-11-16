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

fn calculate_distance<F>(
    input: &str,
    compare: F,
) -> u32
where
    F: Fn(u32, u32) -> bool,
{
    let routes: Vec<Route> = input
        .lines()
        .map(parse)
        .collect::<Result<Vec<_>, _>>()
        .expect("Failed to parse input");

    let places: HashSet<_> = routes
        .iter()
        .flat_map(|r| vec![&r.origin, &r.destination])
        .collect();

    let mut best_distance = if compare(u32::MAX, 0) { 0 } else { u32::MAX };

    for permutation in places.iter().permutations(places.len()) {
        let mut total_distance = 0;
        for window in permutation.windows(2) {
            if let Some(route) = routes.iter().find(|r| {
                (r.origin == **window[0] && r.destination == **window[1])
                    || (r.origin == **window[1] && r.destination == **window[0])
            }) {
                total_distance += route.distance;
            } else {
                total_distance =
                    if compare(u32::MAX, 0) { u32::MAX } else { 0 };
                break;
            }
        }
        if compare(total_distance, best_distance) {
            best_distance = total_distance;
        }
    }

    best_distance
}

pub fn part_1(input: &str) -> i32 {
    calculate_distance(input, |a, b| a < b) as i32
}

pub fn part_2(input: &str) -> i32 {
    calculate_distance(input, |a, b| a > b) as i32
}

generate_tests!(
    2015,           // year
    9,              // day
    part_1,         // part 1 function
    part_2,         // part 2 function
    vec![("London to Dublin = 464\nLondon to Belfast = 518\nDublin to Belfast = 141", 605)], // part 1 examples
    vec![("London to Dublin = 464\nLondon to Belfast = 518\nDublin to Belfast = 141", 982)], // part 2 examples
    Some(207),             // part 1 expected
    Some(804)              // part 2 expected
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
