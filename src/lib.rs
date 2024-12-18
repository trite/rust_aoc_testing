#[macro_use]
pub mod shared;
pub mod year2015;
pub mod year2024;

// Leaving the default example here, it's a nice way to confirm that the project is working as expected.

pub fn add(
    left: u64,
    right: u64,
) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanity_check() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
