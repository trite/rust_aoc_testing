pub fn part_1(_input: &str) -> i32 {
    panic!("Not yet implemented");
}

pub fn part_2(_input: &str) -> i32 {
    panic!("Not yet implemented");
}

fn parse_input(input: &str) -> Option<(Vec<(i32, i32)>, Vec<Vec<i32>>)> {
    let sections: Vec<&str> = input.split("\n\n").collect();
    if sections.len() != 2 {
        return None;
    }

    let section1: Option<Vec<(i32, i32)>> = sections[0]
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split('|').collect();
            if parts.len() == 2 {
                let part1 = parts[0].parse::<i32>().ok()?;
                let part2 = parts[1].parse::<i32>().ok()?;
                Some((part1, part2))
            } else {
                None
            }
        })
        .collect();

    let section2: Option<Vec<Vec<i32>>> = sections[1]
        .lines()
        .map(|line| {
            line.split(',')
                .map(|s| s.parse::<i32>().ok())
                .collect::<Option<Vec<i32>>>()
        })
        .collect();

    section1.and_then(|s1| section2.map(|s2| (s1, s2)))
}

generate_tests!(
    2024,
    5,
    part_1,
    part_2,
    vec![], // part 1 examples
    vec![], // part 2 examples
    None,   // run part 1, expect -1 till we have an answer
    None    // don't run part 2 until we're ready
);

#[cfg(test)]
mod local_tests {
    use super::*;
    use test_case::test_case;

    #[test_case("some input", -1; "example test case")]
    fn parse_test(
        input: &str,
        expected: i32,
    ) {
        assert_eq!(part_1(input), expected);
    }

    #[test]
    fn test_parse_input() {
        let input = "1|2\n3|4\n\n5,6,7\n8,9";
        let expected = Some((
            vec![(1, 2), (3, 4)],
            vec![vec![5, 6, 7], vec![8, 9]],
        ));
        assert_eq!(parse_input(input), expected);
    }
}
