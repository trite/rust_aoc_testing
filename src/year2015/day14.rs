#[derive(Debug, PartialEq)]
struct FlightStats {
    name: String,
    speed: i32,
    duration: i32,
    rest: i32,
}

// Function to parse a line of input into a FlightStats struct
fn parse_flight_stats(input: &str) -> Option<FlightStats> {
    let parts: Vec<&str> = input.split_whitespace().collect();
    if parts.len() == 15 {
        Some(FlightStats {
            name: parts[0].to_string(),
            speed: parts[3].parse().ok()?,
            duration: parts[6].parse().ok()?,
            rest: parts[13].parse().ok()?,
        })
    } else {
        None
    }
}

// Function to calculate the distance traveled after a given number of seconds
fn distance_traveled(
    stats: &FlightStats,
    seconds: i32,
) -> i32 {
    let cycle_time = stats.duration + stats.rest;
    let full_cycles = seconds / cycle_time;
    let remaining_time = seconds % cycle_time;
    let active_time =
        full_cycles * stats.duration + remaining_time.min(stats.duration);
    active_time * stats.speed
}

pub fn part_1(input: &str) -> i32 {
    input
        .lines()
        .filter_map(parse_flight_stats)
        .map(|stats| distance_traveled(&stats, 2503))
        .max()
        .unwrap_or(0)
}

pub fn new_scoring_system(
    input: &str,
    seconds: i32,
) -> i32 {
    let stats: Vec<FlightStats> =
        input.lines().filter_map(parse_flight_stats).collect();
    let mut distances = vec![0; stats.len()];
    let mut points = vec![0; stats.len()];

    for second in 1..=seconds {
        for (i, stat) in stats.iter().enumerate() {
            distances[i] = distance_traveled(stat, second);
        }
        if let Some(max_distance) = distances.iter().max() {
            for (i, &distance) in distances.iter().enumerate() {
                if distance == *max_distance {
                    points[i] += 1;
                }
            }
        }
    }

    *points.iter().max().unwrap_or(&0)
}

pub fn part_2(input: &str) -> i32 {
    new_scoring_system(input, 2503)
}

generate_tests!(
    2015,
    14,
    part_1,
    part_2,
    vec![],
    vec![],
    Some(2660),
    Some(1256)
);

#[cfg(test)]
mod local_tests {
    use super::*;
    use test_case::test_case;

    #[test_case(
        "Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.",
        Some(FlightStats {
            name: "Comet".to_string(),
            speed: 14,
            duration: 10,
            rest: 127
        });
        "parse Comet"
    )]
    #[test_case(
        "Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.",
        Some(FlightStats {
            name: "Dancer".to_string(),
            speed: 16,
            duration: 11,
            rest: 162
        });
        "parse Dancer"
    )]
    fn parse_test(
        input: &str,
        expected: Option<FlightStats>,
    ) {
        assert_eq!(parse_flight_stats(input), expected);
    }

    #[test_case(
        &FlightStats {
            name: "Comet".to_string(),
            speed: 14,
            duration: 10,
            rest: 127
        },
        1000,
        1120;
        "Comet 1000 seconds"
    )]
    #[test_case(
        &FlightStats {
            name: "Dancer".to_string(),
            speed: 16,
            duration: 11,
            rest: 162
        },
        1000,
        1056;
        "Dancer 1000 seconds"
    )]
    fn distance_test(
        stats: &FlightStats,
        seconds: i32,
        expected: i32,
    ) {
        assert_eq!(distance_traveled(stats, seconds), expected);
    }

    #[test_case(
        "Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.\nDancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.",
        1000,
        689;
        "new scoring system 1000 seconds"
    )]
    fn new_scoring_system_test(
        input: &str,
        seconds: i32,
        expected: i32,
    ) {
        assert_eq!(new_scoring_system(input, seconds), expected);
    }
}
