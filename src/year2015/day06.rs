pub fn part_1(input: &str) -> i32 {
    let mut grid = vec![vec![false; 1000]; 1000];

    for line in input.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let (action, start, end) = if parts[0] == "toggle" {
            ("toggle", parts[1], parts[3])
        } else {
            (parts[1], parts[2], parts[4])
        };

        let start_coords: Vec<usize> = start.split(',').map(|x| x.parse().unwrap()).collect();
        let end_coords: Vec<usize> = end.split(',').map(|x| x.parse().unwrap()).collect();

        for x in start_coords[0]..=end_coords[0] {
            for y in start_coords[1]..=end_coords[1] {
                match action {
                    "on" => grid[x][y] = true,
                    "off" => grid[x][y] = false,
                    "toggle" => grid[x][y] = !grid[x][y],
                    _ => panic!("Unknown action"),
                }
            }
        }
    }

    grid.iter().flatten().filter(|&&x| x).count() as i32
}

pub fn part_2(input: &str) -> i32 {
    let mut grid = vec![vec![0; 1000]; 1000];

    for line in input.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let (action, start, end) = if parts[0] == "toggle" {
            ("toggle", parts[1], parts[3])
        } else {
            (parts[1], parts[2], parts[4])
        };

        let start_coords: Vec<usize> = start.split(',').map(|x| x.parse().unwrap()).collect();
        let end_coords: Vec<usize> = end.split(',').map(|x| x.parse().unwrap()).collect();

        for x in start_coords[0]..=end_coords[0] {
            for y in start_coords[1]..=end_coords[1] {
                match action {
                    "on" => grid[x][y] += 1,
                    "off" => {
                        if grid[x][y] > 0 {
                            grid[x][y] -= 1
                        }
                    }
                    "toggle" => grid[x][y] += 2,
                    _ => panic!("Unknown action"),
                }
            }
        }
    }

    grid.iter().flatten().sum()
}

generate_tests!(
    2015,     // year
    6,        // day
    part_1,   // part 1 function
    part_2,   // part 2 function
    vec![],   // part 1 examples
    vec![],   // part 2 examples
    543903,   // part 1 expected
    14687245  // part 2 expected
);
