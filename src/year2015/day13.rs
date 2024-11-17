use itertools::Itertools;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
struct Happiness {
    person: String,
    neighbor: String,
    change: i32,
}

impl Happiness {
    fn parse(line: &str) -> Option<Self> {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() != 11 {
            return None;
        }

        let person = parts[0].to_string();
        let neighbor = parts[10].trim_end_matches('.').to_string();
        let change = match parts[2] {
            "gain" => parts[3].parse::<i32>().ok()?,
            "lose" => -parts[3].parse::<i32>().ok()?,
            _ => return None,
        };

        Some(Happiness {
            person,
            neighbor,
            change,
        })
    }
}

fn calculate_max_happiness(happiness_list: Vec<Happiness>) -> i32 {
    let mut happiness_map: HashMap<(&str, &str), i32> = HashMap::new();
    let mut people: Vec<&str> = Vec::new();

    for happiness in &happiness_list {
        happiness_map
            .insert((&happiness.person, &happiness.neighbor), happiness.change);
        if !people.contains(&happiness.person.as_str()) {
            people.push(&happiness.person);
        }
    }

    let mut max_happiness = i32::MIN;

    for permutation in people.iter().permutations(people.len()) {
        let mut total_happiness = 0;

        for i in 0..permutation.len() {
            let person = permutation[i];
            let neighbor = permutation[(i + 1) % permutation.len()];
            total_happiness +=
                happiness_map.get(&(person, neighbor)).unwrap_or(&0);
            total_happiness +=
                happiness_map.get(&(neighbor, person)).unwrap_or(&0);
        }

        if total_happiness > max_happiness {
            max_happiness = total_happiness;
        }
    }

    max_happiness
}

pub fn part_1(input: &str) -> i32 {
    let happiness_list: Vec<Happiness> =
        input.lines().map(Happiness::parse).flatten().collect();

    calculate_max_happiness(happiness_list)
}

pub fn part_2(_input: &str) -> i32 {
    panic!("Not yet implemented");
}

generate_tests!(
    2015,
    13,
    part_1,
    part_2,
    vec![(
        "Alice would gain 54 happiness units by sitting next to Bob.\nAlice would lose 79 happiness units by sitting next to Carol.\nAlice would lose 2 happiness units by sitting next to David.\nBob would gain 83 happiness units by sitting next to Alice.\nBob would lose 7 happiness units by sitting next to Carol.\nBob would lose 63 happiness units by sitting next to David.\nCarol would lose 62 happiness units by sitting next to Alice.\nCarol would gain 60 happiness units by sitting next to Bob.\nCarol would gain 55 happiness units by sitting next to David.\nDavid would gain 46 happiness units by sitting next to Alice.\nDavid would lose 7 happiness units by sitting next to Bob.\nDavid would gain 41 happiness units by sitting next to Carol.",
        330
    )],
    vec![],
    Some(709),
    None
);

#[cfg(test)]
mod local_tests {
    use super::*;
    use test_case::test_case;

    #[test_case(
        "Alice would gain 54 happiness units by sitting next to Bob.",
        Some(Happiness {
            person: "Alice".to_string(),
            neighbor: "Bob".to_string(),
            change: 54
        });
        "parse test 1"
    )]
    #[test_case(
        "Alice would lose 79 happiness units by sitting next to Carol.",
        Some(Happiness {
            person: "Alice".to_string(),
            neighbor: "Carol".to_string(),
            change: -79
        });
        "parse test 2"
    )]
    #[test_case(
        "Alice would lose 2 happiness units by sitting next to David.",
        Some(Happiness {
            person: "Alice".to_string(),
            neighbor: "David".to_string(),
            change: -2
        });
        "parse test 3"
    )]
    #[test_case(
        "Bob would gain 83 happiness units by sitting next to Alice.",
        Some(Happiness {
            person: "Bob".to_string(),
            neighbor: "Alice".to_string(),
            change: 83
        });
        "parse test 4"
    )]
    #[test_case(
        "Bob would lose 7 happiness units by sitting next to Carol.",
        Some(Happiness {
            person: "Bob".to_string(),
            neighbor: "Carol".to_string(),
            change: -7
        });
        "parse test 5"
    )]
    #[test_case(
        "Bob would lose 63 happiness units by sitting next to David.",
        Some(Happiness {
            person: "Bob".to_string(),
            neighbor: "David".to_string(),
            change: -63
        });
        "parse test 6"
    )]
    #[test_case(
        "Carol would lose 62 happiness units by sitting next to Alice.",
        Some(Happiness {
            person: "Carol".to_string(),
            neighbor: "Alice".to_string(),
            change: -62
        });
        "parse test 7"
    )]
    #[test_case(
        "Carol would gain 60 happiness units by sitting next to Bob.",
        Some(Happiness {
            person: "Carol".to_string(),
            neighbor: "Bob".to_string(),
            change: 60
        });
        "parse test 8"
    )]
    #[test_case(
        "Carol would gain 55 happiness units by sitting next to David.",
        Some(Happiness {
            person: "Carol".to_string(),
            neighbor: "David".to_string(),
            change: 55
        });
        "parse test 9"
    )]
    #[test_case(
        "David would gain 46 happiness units by sitting next to Alice.",
        Some(Happiness {
            person: "David".to_string(),
            neighbor: "Alice".to_string(),
            change: 46
        });
        "parse test 10"
    )]
    #[test_case(
        "David would lose 7 happiness units by sitting next to Bob.",
        Some(Happiness {
            person: "David".to_string(),
            neighbor: "Bob".to_string(),
            change: -7
        });
        "parse test 11"
    )]
    #[test_case(
        "David would gain 41 happiness units by sitting next to Carol.",
        Some(Happiness {
            person: "David".to_string(),
            neighbor: "Carol".to_string(),
            change: 41
        });
        "parse test 12"
    )]
    fn parse_test(
        input: &str,
        expected: Option<Happiness>,
    ) {
        assert_eq!(Happiness::parse(input), expected);
    }

    #[test]
    fn test_calculate_max_happiness() {
        let happiness_list = vec![
            Happiness::parse(
                "Alice would gain 54 happiness units by sitting next to Bob.",
            )
            .unwrap(),
            Happiness::parse(
                "Alice would lose 79 happiness units by sitting next to Carol.",
            )
            .unwrap(),
            Happiness::parse(
                "Alice would lose 2 happiness units by sitting next to David.",
            )
            .unwrap(),
            Happiness::parse(
                "Bob would gain 83 happiness units by sitting next to Alice.",
            )
            .unwrap(),
            Happiness::parse(
                "Bob would lose 7 happiness units by sitting next to Carol.",
            )
            .unwrap(),
            Happiness::parse(
                "Bob would lose 63 happiness units by sitting next to David.",
            )
            .unwrap(),
            Happiness::parse(
                "Carol would lose 62 happiness units by sitting next to Alice.",
            )
            .unwrap(),
            Happiness::parse(
                "Carol would gain 60 happiness units by sitting next to Bob.",
            )
            .unwrap(),
            Happiness::parse(
                "Carol would gain 55 happiness units by sitting next to David.",
            )
            .unwrap(),
            Happiness::parse(
                "David would gain 46 happiness units by sitting next to Alice.",
            )
            .unwrap(),
            Happiness::parse(
                "David would lose 7 happiness units by sitting next to Bob.",
            )
            .unwrap(),
            Happiness::parse(
                "David would gain 41 happiness units by sitting next to Carol.",
            )
            .unwrap(),
        ];

        assert_eq!(calculate_max_happiness(happiness_list), 330);
    }
}
