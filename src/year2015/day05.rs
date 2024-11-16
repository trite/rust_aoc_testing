pub fn part_1(input: &str) -> i32 {
    input
        .lines()
        .filter(|line| {
            let mut vowels = 0;
            let mut double = false;
            let mut prev = ' ';
            for c in line.chars() {
                match c {
                    'a' | 'e' | 'i' | 'o' | 'u' => vowels += 1,
                    _ => {}
                }
                if c == prev {
                    double = true;
                }
                if (prev == 'a' && c == 'b')
                    || (prev == 'c' && c == 'd')
                    || (prev == 'p' && c == 'q')
                    || (prev == 'x' && c == 'y')
                {
                    return false;
                }
                prev = c;
            }
            vowels >= 3 && double
        })
        .count() as i32
}

pub fn part_2(input: &str) -> i32 {
    input
        .lines()
        .filter(|line| {
            let binding = line.chars().collect::<Vec<_>>();
            let pairs: Vec<_> = binding.windows(2).collect();
            let trips: Vec<_> = binding.windows(3).collect();

            let repeat_pair = pairs
                .iter()
                .enumerate()
                .any(|(i, pair)| pairs.iter().skip(i + 2).any(|p| p == pair));

            let trip_pair = trips
                .iter()
                .any(|trip| trip[0] == trip[2] && trip[0] != trip[1]);

            repeat_pair && trip_pair
        })
        .count() as i32
}

generate_tests!(
    2015,   // year
    5,      // day
    part_1, // part 1 function
    part_2, // part 2 function
    vec![(
        "ugknbfddgicrmopn\naaa\njchzalrnumimnmhp\nhaegwjzuvuyypxyu\ndvszwmarrgswjxmb",
        2
    )], // part 1 examples
    vec![(
        "qjhvhtzxzqqjkmpb\nxxyxx\nuurcxstgmygtbstg\nieodomkazucvgmuy",
        2
    )], // part 2 examples
    Some(258),    // part 1 expected
    Some(53)      // part 2 expected
);
