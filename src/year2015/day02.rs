pub fn part_1(input: &str) -> i32 {
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

pub fn part_2(input: &str) -> i32 {
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

#[allow(unused_imports)]
#[allow(dead_code)]
mod tests {
    use super::*;
    use crate::shared::test_input::{run_examples, run_part_against_input};
    use std::fs;

    const YEAR: i32 = 2015;
    const DAY: i32 = 2;

    #[test]
    fn part_1_examples() {
        run_examples(vec![("2x3x4", 58), ("1x1x10", 43)], part_1);
    }

    #[test]
    fn part_1_input() {
        run_part_against_input(YEAR, DAY, part_1, 1606483);
    }

    #[test]
    fn part_2_examples() {
        run_examples(vec![("2x3x4", 34), ("1x1x10", 14)], part_2);
    }

    #[test]
    fn part_2_input() {
        run_part_against_input(YEAR, DAY, part_2, 3842356);
    }
}
