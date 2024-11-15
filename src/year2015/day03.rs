pub fn part_1(input: &str) -> i32 {
    let mut x = 0;
    let mut y = 0;
    let mut visited = std::collections::HashSet::new();
    visited.insert((x, y));
    for c in input.chars() {
        match c {
            '^' => y += 1,
            'v' => y -= 1,
            '>' => x += 1,
            '<' => x -= 1,
            _ => panic!("Invalid character"),
        }
        visited.insert((x, y));
    }
    visited.len() as i32
}

pub fn part_2(input: &str) -> i32 {
    let mut santa_x = 0;
    let mut santa_y = 0;
    let mut robot_x = 0;
    let mut robot_y = 0;
    let mut visited = std::collections::HashSet::new();

    // Insert starting coord manually because I'm lazy
    visited.insert((0, 0));

    for (index, c) in input.chars().enumerate() {
        let (x, y) = if index % 2 == 0 {
            (&mut santa_x, &mut santa_y)
        } else {
            (&mut robot_x, &mut robot_y)
        };

        match c {
            '^' => *y += 1,
            'v' => *y -= 1,
            '>' => *x += 1,
            '<' => *x -= 1,
            _ => panic!("Invalid character"),
        }

        visited.insert((*x, *y));
    }

    visited.len() as i32
}

generate_tests!(
    2015,
    3,
    part_1,
    part_2,
    vec![(">", 2), ("^>v<", 4), ("^v^v^v^v^v", 2)],
    vec![("^v", 3), ("^>v<", 3), ("^v^v^v^v^v", 11)],
    2081,
    2341
);
