use std::num::ParseIntError;

enum Operation {
    Assign(ValOrVar, String),
    And(String, String, String),
    Or(String, String, String),
    LShift(String, i32, String),
    RShift(String, i32, String),
    Not(String, String),
}

enum ValOrVar {
    Value(i32),
    Variable(String),
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
        [source, "->", dest] => {
            if let Ok(value) = source.parse() {
                Ok(Operation::Assign(ValOrVar::Value(value), dest.to_string()))
            } else {
                Ok(Operation::Assign(
                    ValOrVar::Variable(source.to_string()),
                    dest.to_string(),
                ))
            }
        }
        _ => Err(ParseError::InvalidInput),
    }
}

pub fn part_1(input: &str) -> i32 {
    let mut operations: Vec<Result<Operation, ParseError>> = Vec::new();
    for line in input.lines() {
        match parse_line(line) {
            Ok(op) => operations.push(Ok(op)),
            Err(ParseError::InvalidInput) => {
                eprintln!("Invalid input: {}", line);
                panic!("Invalid input");
            }
            Err(ParseError::ParseIntError(err)) => {
                eprintln!("ParseIntError: {}, for input: {}", err, line);
                panic!("Invalid input");
            }
        }
    }

    let mut values = std::collections::HashMap::new();
    let mut resolved = std::collections::HashMap::new();

    while resolved.len() < operations.len() {
        for operation in &operations {
            match operation {
                Ok(Operation::Assign(operand, dest)) => match operand {
                    &ValOrVar::Value(value) => {
                        values.insert(dest.clone(), value);
                    }
                    &ValOrVar::Variable(var) => {
                        if let Some(value) = values.get(&var) {
                            values.insert(dest.clone(), *value);
                        }
                    }
                },
                Ok(Operation::And(left, right, dest)) => {
                    if values.contains_key(left) && values.contains_key(right) {
                        values
                            .insert(dest.clone(), values[left] & values[right]);
                    }
                }
                Ok(Operation::Or(left, right, dest)) => {
                    if values.contains_key(left) && values.contains_key(right) {
                        values
                            .insert(dest.clone(), values[left] | values[right]);
                    }
                }
                Ok(Operation::LShift(left, amount, dest)) => {
                    if values.contains_key(left) {
                        values.insert(dest.clone(), values[left] << amount);
                    }
                }
                Ok(Operation::RShift(left, amount, dest)) => {
                    if values.contains_key(left) {
                        values.insert(dest.clone(), values[left] >> amount);
                    }
                }
                Ok(Operation::Not(value, dest)) => {
                    if values.contains_key(value) {
                        values.insert(dest.clone(), !values[value].clone());
                    }
                }
                Err(_) => {
                    /* Already handled by the "Invalid Input" check(s) above */
                }
            }
        }
    }

    values["a"]
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
