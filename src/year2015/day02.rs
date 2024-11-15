pub fn part_1(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            let mut dimensions =
                line.split('x').map(|x| x.parse::<i32>().unwrap());
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
            let mut dimensions =
                line.split('x').map(|x| x.parse::<i32>().unwrap());
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

generate_tests!(
    2015,
    2,
    part_1,
    part_2,
    vec![("2x3x4", 58), ("1x1x10", 43)],
    vec![("2x3x4", 34), ("1x1x10", 14)],
    1606483,
    3842356
);
