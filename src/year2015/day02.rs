fn part_1(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            let mut dimensions = line.split('x').map(|x| x.parse::<i32>().unwrap());
            let l = dimensions.next().unwrap();
            let w = dimensions.next().unwrap();
            let h = dimensions.next().unwrap();
            let lw = l * w;
            let wh = w * h;
            let hl = h * l;
            2 * lw + 2 * wh + 2 * hl + lw.min(wh).min(hl)
        })
        .sum()
}

fn part_2(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            let mut dimensions = line.split('x').map(|x| x.parse::<i32>().unwrap());
            let l = dimensions.next().unwrap();
            let w = dimensions.next().unwrap();
            let h = dimensions.next().unwrap();
            let lw = l + w;
            let wh = w + h;
            let hl = h + l;
            let min_side = lw.min(wh).min(hl);
            let volume = l * w * h;
            2 * min_side + volume
        })
        .sum()
}

mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn part_1_examples() {
        vec![("2x3x4", 58), ("1x1x10", 43)]
            .iter()
            .enumerate()
            .for_each(|(index, (input, expected))| {
                assert_eq!(part_1(input), *expected, "Test failed for index: {}", index);
            });
    }

    #[test]
    fn part_1_input() {
        let path = "src/year2015/day02.txt";
        let input = fs::read_to_string(path).expect(&format!("Failed to read from {}", path));
        let result = part_1(&input);
        assert_eq!(result, 1606483);
    }

    #[test]
    fn part_2_examples() {
        vec![("2x3x4", 34), ("1x1x10", 14)]
            .iter()
            .enumerate()
            .for_each(|(index, (input, expected))| {
                assert_eq!(part_2(input), *expected, "Test failed for index: {}", index);
            });
    }

    #[test]
    fn part_2_input() {
        let path = "src/year2015/day02.txt";
        let input = fs::read_to_string(path).expect(&format!("Failed to read from {}", path));
        let result = part_2(&input);
        // Copilot even guessed the result # here, which seems odd that it
        //   was the correct answer for my input... was it random luck or
        //   did it examine the day01.txt file?
        assert_eq!(result, 3842356);
    }
}
