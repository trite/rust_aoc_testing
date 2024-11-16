pub fn parse_line(input: &str) -> (i32, i32) {
    let mut char_code_count = 0;
    let mut memory_count = 0;
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        char_code_count += 1;
        if c == '\\' {
            if let Some(next) = chars.peek() {
                match next {
                    '\\' | '"' => {
                        chars.next();
                        char_code_count += 1;
                        memory_count += 1;
                    }
                    'x' => {
                        chars.next();
                        chars.next();
                        chars.next();
                        char_code_count += 3;
                        memory_count += 1;
                    }
                    _ => {}
                }
            }
        } else if c != '"' {
            memory_count += 1;
        }
    }

    (char_code_count, memory_count)
}

pub fn part_1(input: &str) -> i32 {
    let (char_code_sum, memory_sum): (i32, i32) = input
        .lines()
        .map(parse_line)
        .fold((0, 0), |(char_acc, mem_acc), (char_count, mem_count)| {
            (char_acc + char_count, mem_acc + mem_count)
        });
    char_code_sum - memory_sum
}

pub fn part_2(input: &str) -> i32 {
    let original_char_code_sum: i32 =
        input.lines().map(|line| parse_line(line).0).sum();

    let escaped_char_code_sum: i32 = input
        .lines()
        .map(|line| {
            let escaped = line.replace("\\", "\\\\").replace("\"", "\\\"");
            parse_line(&format!("\"{}\"", escaped)).0
        })
        .sum();

    escaped_char_code_sum - original_char_code_sum
}

generate_tests!(
    2015,                                                   // year
    8,                                                      // day
    part_1,                                                 // part 1 function
    part_2,                                                 // part 2 function
    vec![("\"\"\n\"abc\"\n\"aaa\\\"aaa\"\n\"\\x27\"", 12)], // part 1 examples
    vec![("\"\"\n\"abc\"\n\"aaa\\\"aaa\"\n\"\\x27\"", 19)], // part 2 examples
    Some(1342),                                             // part 1 expected
    Some(2074)                                              // part 2 expected
);
