use std::num::ParseIntError;

enum Operation {
    Assign(Operand, String),
    And(String, String, String),
    Or(String, String, String),
    LShift(String, i32, String),
    RShift(String, i32, String),
    Not(String, String),
}

enum Operand {
    Value(i32),
    Wire(String),
}

#[derive(Debug)]
enum ParseError {
    InvalidInput,
    ParseIntError(ParseIntError),
}

impl From<ParseIntError> for ParseError {
    fn from(err: ParseIntError) -> ParseError {
        ParseError::ParseIntError(err)
    }
}

fn parse_line(line: &str) -> Result<Operation, ParseError> {
    let parts: Vec<&str> = line.split_whitespace().collect();

    match parts.as_slice() {
        [left, "AND", right, "->", dest] => Ok(Operation::And(
            left.to_string(),
            right.to_string(),
            dest.to_string(),
        )),
        [left, "OR", right, "->", dest] => Ok(Operation::Or(
            left.to_string(),
            right.to_string(),
            dest.to_string(),
        )),
        [left, "LSHIFT", amount, "->", dest] => amount
            .parse()
            .map(|amount| {
                Operation::LShift(left.to_string(), amount, dest.to_string())
            })
            .map_err(ParseError::from),
        [left, "RSHIFT", amount, "->", dest] => amount
            .parse()
            .map(|amount| {
                Operation::RShift(left.to_string(), amount, dest.to_string())
            })
            .map_err(ParseError::from),
        ["NOT", value, "->", dest] => {
            Ok(Operation::Not(value.to_string(), dest.to_string()))
        }
        [value, "->", dest] => {
            if let Ok(num) = value.parse() {
                Ok(Operation::Assign(Operand::Value(num), dest.to_string()))
            } else {
                Ok(Operation::Assign(
                    Operand::Wire(value.to_string()),
                    dest.to_string(),
                ))
            }
        }
        _ => Err(ParseError::InvalidInput),
    }
}

pub fn part_1(input: &str) -> i32 {
    panic!("Not yet implemented");
}

pub fn part_2(input: &str) -> i32 {
    panic!("Not yet implemented");
}

generate_tests!(
    2015,   // year
    7,      // day
    part_1, // part 1 function
    part_2, // part 2 function
    vec![], // part 1 examples
    vec![], // part 2 examples
    46065,  // part 1 expected
    14134   // part 2 expected
);
