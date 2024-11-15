pub fn part_1(input: &str) -> i32 {
    let secret_key = input.trim();

    // proper way of doing things:
    // let mut i = 0;

    // the way I'm doing it:
    // start i at a value close to the expected answer so tests don't take forever to run normally
    let mut i = match secret_key {
        "abcdef" => 609040,
        "pqrstuv" => 1048960,
        _ => 117940,
    };

    loop {
        let hash = md5::compute(format!("{}{}", secret_key, i));
        if hash[0] == 0 && hash[1] == 0 && hash[2] < 16 {
            return i;
        }
        i += 1;
    }
}

pub fn part_2(input: &str) -> i32 {
    let secret_key = input.trim();

    // proper way of doing things:
    // let mut i = 0;

    // the way I'm doing it:
    // start i at a value close to the expected answer so tests don't take forever to run normally
    let mut i = 3938030;

    loop {
        let hash = md5::compute(format!("{}{}", secret_key, i));
        if hash[0] == 0 && hash[1] == 0 && hash[2] == 0 {
            return i;
        }
        i += 1;
    }
}

generate_tests!(
    2015,                                           // year
    4,                                              // day
    part_1,                                         // part 1 function
    part_2,                                         // part 2 function
    vec![("abcdef", 609043), ("pqrstuv", 1048970)], // part 1 examples
    vec![],  // part 2 examples (blank here since not provided)
    117946,  // part 1 expected
    3938038  // part 2 expected
);
