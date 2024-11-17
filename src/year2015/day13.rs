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

pub fn part_1(_input: &str) -> i32 {
    panic!("Not yet implemented");
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
        -1
    )], // part 1 examples
    vec![],         // part 2 examples
    Some(-1), // run part 1, expect -1 since we don't know the right answer yet
    None      // don't run part 2 until we're ready
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
}
