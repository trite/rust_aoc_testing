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
            let mut pairs = std::collections::HashSet::new();
            let mut repeat = false;
            let mut prev = ' ';
            let mut prev_prev = ' ';
            for c in line.chars() {
                if pairs.contains(&(prev, c)) {
                    repeat = true;
                }
                if prev_prev == c {
                    repeat = true;
                }
                pairs.insert((prev, c));
                prev_prev = prev;
                prev = c;
            }
            repeat
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
    258,    // part 1 expected
    53      // part 2 expected
);
