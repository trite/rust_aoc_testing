use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Ingredient {
    name: String,
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

fn parse_ingredient(input: &str) -> Vec<Ingredient> {
    input
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split(": ").collect();
            let name = parts[0].to_string();
            let attributes: Vec<i32> = parts[1]
                .split(", ")
                .map(|attr| {
                    attr.split_whitespace().last().unwrap().parse().unwrap()
                })
                .collect();
            Ingredient {
                name,
                capacity: attributes[0],
                durability: attributes[1],
                flavor: attributes[2],
                texture: attributes[3],
                calories: attributes[4],
            }
        })
        .collect()
}

pub fn part_1(_input: &str) -> i32 {
    panic!("Not yet implemented");
}

pub fn part_2(_input: &str) -> i32 {
    panic!("Not yet implemented");
}

generate_tests!(
    2015,
    15,
    part_1,
    part_2,
    vec![], // part 1 examples
    vec![], // part 2 examples
    None,   // run part 1, expect -1 till we have an answer
    None    // don't run part 2 until we're ready
);

#[cfg(test)]
mod local_tests {
    use super::*;
    use test_case::test_case;

    #[test_case("Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8", Ingredient {
        name: "Butterscotch".to_string(),
        capacity: -1,
        durability: -2,
        flavor: 6,
        texture: 3,
        calories: 8
    }; "Butterscotch test case")]
    #[test_case("Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3", Ingredient {
        name: "Cinnamon".to_string(),
        capacity: 2,
        durability: 3,
        flavor: -2,
        texture: -1,
        calories: 3
    }; "Cinnamon test case")]
    fn parse_test(
        input: &str,
        expected: Ingredient,
    ) {
        let ingredients = parse_ingredient(input);
        assert_eq!(ingredients.len(), 1);
        assert_eq!(ingredients[0], expected);
    }
}
