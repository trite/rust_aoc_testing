use std::num::ParseIntError;

#[derive(Clone, Debug)]
enum ResolveState {
    Resolved(i32),
    Unresolved(Operation),
}

#[derive(Clone, Debug)]
enum Operation {
    Assign(Operand, String),
    And(Operand, String, String),
    Or(String, String, String),
    LShift(String, i32, String),
    RShift(String, i32, String),
    Not(String, String),
}

#[derive(Clone, Debug)]
enum Operand {
    Value(i32),
    Wire(String),
}

#[derive(Debug)]
enum ParseError {
    InvalidInput,
    #[allow(dead_code)]
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
        [left, "AND", right, "->", dest] => {
            let left_operand = if let Ok(num) = left.parse() {
                Operand::Value(num)
            } else {
                Operand::Wire(left.to_string())
            };
            Ok(Operation::And(
                left_operand,
                right.to_string(),
                dest.to_string(),
            ))
        }
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

fn resolve_operations(
    operations: &[Operation],
    initial_state: Option<&std::collections::HashMap<String, ResolveState>>,
) -> std::collections::HashMap<String, ResolveState> {
    let mut state: std::collections::HashMap<String, ResolveState> =
        std::collections::HashMap::new();

    for operation in operations {
        match operation {
            Operation::Assign(Operand::Value(value), dest) => {
                state.insert(dest.clone(), ResolveState::Resolved(*value));
            }
            Operation::Assign(_, dest)
            | Operation::And(_, _, dest)
            | Operation::Or(_, _, dest)
            | Operation::LShift(_, _, dest)
            | Operation::RShift(_, _, dest)
            | Operation::Not(_, dest) => {
                state.insert(
                    dest.clone(),
                    ResolveState::Unresolved(operation.clone()),
                );
            }
        }
    }

    if let Some(initial_state) = initial_state {
        state.extend(initial_state.clone());
    }

    loop {
        let mut progress = false;

        for (_key, value) in state.clone() {
            if let ResolveState::Unresolved(operation) = value {
                match operation {
                    Operation::Assign(Operand::Wire(ref var), ref dest) => {
                        if let Some(ResolveState::Resolved(val)) =
                            state.get(var)
                        {
                            state.insert(
                                dest.clone(),
                                ResolveState::Resolved(*val),
                            );
                            progress = true;
                        }
                    }
                    Operation::And(
                        Operand::Wire(ref left),
                        ref right,
                        ref dest,
                    ) => {
                        if let (
                            Some(ResolveState::Resolved(left_val)),
                            Some(ResolveState::Resolved(right_val)),
                        ) = (state.get(left), state.get(right))
                        {
                            state.insert(
                                dest.clone(),
                                ResolveState::Resolved(left_val & right_val),
                            );
                            progress = true;
                        }
                    }
                    Operation::And(
                        Operand::Value(left_val),
                        ref right,
                        ref dest,
                    ) => {
                        if let Some(ResolveState::Resolved(right_val)) =
                            state.get(right)
                        {
                            state.insert(
                                dest.clone(),
                                ResolveState::Resolved(left_val & right_val),
                            );
                            progress = true;
                        }
                    }
                    Operation::Or(ref left, ref right, ref dest) => {
                        if let (
                            Some(ResolveState::Resolved(left_val)),
                            Some(ResolveState::Resolved(right_val)),
                        ) = (state.get(left), state.get(right))
                        {
                            state.insert(
                                dest.clone(),
                                ResolveState::Resolved(left_val | right_val),
                            );
                            progress = true;
                        }
                    }
                    Operation::LShift(ref left, amount, ref dest) => {
                        if let Some(ResolveState::Resolved(left_val)) =
                            state.get(left)
                        {
                            state.insert(
                                dest.clone(),
                                ResolveState::Resolved(left_val << amount),
                            );
                            progress = true;
                        }
                    }
                    Operation::RShift(ref left, amount, ref dest) => {
                        if let Some(ResolveState::Resolved(left_val)) =
                            state.get(left)
                        {
                            state.insert(
                                dest.clone(),
                                ResolveState::Resolved(left_val >> amount),
                            );
                            progress = true;
                        }
                    }
                    Operation::Not(ref value, ref dest) => {
                        if let Some(ResolveState::Resolved(val)) =
                            state.get(value)
                        {
                            state.insert(
                                dest.clone(),
                                ResolveState::Resolved(!val),
                            );
                            progress = true;
                        }
                    }
                    _ => {}
                }
            }
        }

        if !progress {
            println!("{:?}", state);
            panic!("No progress made, possible circular dependency or unresolved operations");
        }

        if state
            .values()
            .all(|v| matches!(v, ResolveState::Resolved(_)))
        {
            break;
        }
    }

    state
}

pub fn part_1(input: &str) -> i32 {
    let operations: Vec<Operation> = input
        .lines()
        .map(|line| parse_line(line).expect("Invalid input"))
        .collect();

    let state = resolve_operations(&operations, None);

    if let Some(ResolveState::Resolved(result)) = state.get("a") {
        *result
    } else {
        panic!("Value for 'a' not found")
    }
}

pub fn part_2(input: &str) -> i32 {
    let operations: Vec<Operation> = input
        .lines()
        .map(|line| parse_line(line).expect("Invalid input"))
        .collect();

    let state = resolve_operations(&operations, None);

    if let Some(ResolveState::Resolved(a_value)) = state.get("a") {
        let mut new_state_1: std::collections::HashMap<String, ResolveState> =
            std::collections::HashMap::new();
        new_state_1.insert("b".to_string(), ResolveState::Resolved(*a_value));

        let new_state = resolve_operations(&operations, Some(&new_state_1));

        if let Some(ResolveState::Resolved(result)) = new_state.get("a") {
            *result
        } else {
            panic!("Value for 'a' not found (second pass)")
        }
    } else {
        panic!("Value for 'a' not found (first pass)")
    }
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
