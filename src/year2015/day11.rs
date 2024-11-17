pub fn part_1(input: &str) -> String {
    next_valid_password(input)
}

pub fn part_2(_input: &str) -> i32 {
    panic!("Not yet implemented");
}

fn is_valid_password(password: &str) -> bool {
    fn has_increasing_straight(s: &str) -> bool {
        s.as_bytes()
            .windows(3)
            .any(|w| w[0] + 1 == w[1] && w[1] + 1 == w[2])
    }

    fn has_no_invalid_chars(s: &str) -> bool {
        !s.contains('i') && !s.contains('l') && !s.contains('o')
    }

    fn has_two_non_overlapping_pairs(s: &str) -> bool {
        let mut pairs = 0;
        let mut i = 0;
        while i < s.len() - 1 {
            if s.as_bytes()[i] == s.as_bytes()[i + 1] {
                pairs += 1;
                i += 1; // skip the next character
                if pairs == 2 {
                    return true;
                }
            }
            i += 1;
        }
        false
    }

    has_increasing_straight(password)
        && has_no_invalid_chars(password)
        && has_two_non_overlapping_pairs(password)
}

fn next_password_value(password: &str) -> String {
    let mut password = password.to_string().into_bytes();
    let mut i = password.len() - 1;
    while i >= 0 {
        if password[i] == b'z' {
            password[i] = b'a';
            i -= 1;
        } else {
            password[i] += 1;
            break;
        }
    }
    String::from_utf8(password).unwrap()
}

fn next_valid_password(password: &str) -> String {
    let mut next_pw = next_password_value(password);
    while !is_valid_password(&next_pw) {
        next_pw = next_password_value(&next_pw);
    }
    next_pw
}

generate_tests!(
    2015,                         // year
    11,                           // day
    part_1,                       // part 1 function
    part_2,                       // part 2 function
    vec![],                       // part 1 examples
    vec![],                       // part 2 examples
    Some("hxbxxyzz".to_string()), // run part 1, expect -1 since we don't know the right answer yet
    None                          // don't run part 2 until we're ready
);

#[cfg(test)]
mod local_tests {
    use super::*;
    use test_case::test_case;

    #[test_case("hijklmmn", false; "pw validation 1")]
    #[test_case("abbceffg", false; "pw validation 2")]
    #[test_case("abbcegjk", false; "pw validation 3")]
    #[test_case("abcdefgh", false; "pw validation 4")]
    #[test_case("abcdffaa", true; "pw validation 5")]
    #[test_case("ghijklmn", false; "pw validation 6")]
    #[test_case("ghjaabcc", true; "pw validation 7")]
    fn password_validate_test(
        input: &str,
        expected: bool,
    ) {
        assert_eq!(is_valid_password(input), expected);
    }

    #[test_case("abcdefgh", "abcdefgi"; "next pw test 1")]
    #[test_case("ghijklmn", "ghijklmo"; "next pw test 2")]
    fn next_password_test(
        input: &str,
        expected: &str,
    ) {
        assert_eq!(next_password_value(input), expected);
    }

    #[test_case("abcdefgh", "abcdffaa"; "next valid pw test 1")]
    #[test_case("ghijklmn", "ghjaabcc"; "next valid pw test 2")]
    fn next_valid_password_test(
        input: &str,
        expected: &str,
    ) {
        assert_eq!(next_valid_password(input), expected);
    }
}
