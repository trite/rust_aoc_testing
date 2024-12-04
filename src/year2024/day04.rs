// Part 1 took ~15 minutes
// No prompt for this one
pub fn part_1(input: &str) -> i32 {
    let grid = string_to_grid(input);
    count_occurrences(grid, "XMAS")
}

pub fn part_2(_input: &str) -> i32 {
    panic!("Not yet implemented");
}

/* Prompt:
   Create a function that takes 2 strings as input. It should break the first string into lines and each line down by characters, and build a 2 dimensional vector out of it. From there it should count how many times the second input string appears in any direction (including diagonally, backwards, and diagonal+backwards), returning the total count as its output.
*/
fn count_occurrences(
    grid: Vec<Vec<char>>,
    word: &str,
) -> i32 {
    let directions = [
        (0, 1),
        (1, 0),
        (1, 1),
        (1, -1),
        (0, -1),
        (-1, 0),
        (-1, -1),
        (-1, 1),
    ];
    let word_chars: Vec<char> = word.chars().collect();
    let mut count = 0;

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            for &(dx, dy) in &directions {
                let mut k = 0;
                while k < word_chars.len() {
                    let x = i as isize + k as isize * dx;
                    let y = j as isize + k as isize * dy;
                    if x < 0
                        || y < 0
                        || x >= grid.len() as isize
                        || y >= grid[i].len() as isize
                    {
                        break;
                    }
                    if grid[x as usize][y as usize] != word_chars[k] {
                        break;
                    }
                    k += 1;
                }
                if k == word_chars.len() {
                    count += 1;
                }
            }
        }
    }
    count
}

/* Prompt:
   Create a function that will take an input string and break it into the 2d vector needed for count_occurrences.
*/
fn string_to_grid(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

generate_tests!(
    2024,
    4,
    part_1,
    part_2,
    vec![("MMMSXXMASM\nMSAMXMSMSA\nAMXSXMAAMM\nMSAMASMSMX\nXMASAMXAMM\nXXAMMXXAMA\nSMSMSASXSS\nSAXAMASAAA\nMAMMMXMMMM\nMXMXAXMASX", 18)], // part 1 examples
    vec![],         // part 2 examples
    Some(2618),       // run part 1, expect -1 till we have an answer
    None            // don't run part 2 until we're ready
);

#[cfg(test)]
mod local_tests {
    use super::*;
    // use test_case::test_case;

    // Didn't generate automatically, had to prompt
    /* Prompt:
       Add a test case as well, using this as the first input:

       MMMSXXMASM\nMSAMXMSMSA\nAMXSXMAAMM\nMSAMASMSMX\nXMASAMXAMM\nXXAMMXXAMA\nSMSMSASXSS\nSAXAMASAAA\nMAMMMXMMMM\nMXMXAXMASX

       and this as the second:

       XMAS

       the result should be 18
    */
    #[test]
    fn test_count_occurrences() {
        let grid = vec![
            vec!['M', 'M', 'M', 'S', 'X', 'X', 'M', 'A', 'S', 'M'],
            vec!['M', 'S', 'A', 'M', 'X', 'M', 'S', 'M', 'S', 'A'],
            vec!['A', 'M', 'X', 'S', 'X', 'M', 'A', 'A', 'M', 'M'],
            vec!['M', 'S', 'A', 'M', 'A', 'S', 'M', 'S', 'M', 'X'],
            vec!['X', 'M', 'A', 'S', 'A', 'M', 'X', 'A', 'M', 'M'],
            vec!['X', 'X', 'A', 'M', 'M', 'X', 'X', 'A', 'M', 'A'],
            vec!['S', 'M', 'S', 'M', 'S', 'A', 'S', 'X', 'S', 'S'],
            vec!['S', 'A', 'X', 'A', 'M', 'A', 'S', 'A', 'A', 'A'],
            vec!['M', 'A', 'M', 'M', 'M', 'X', 'M', 'M', 'M', 'M'],
            vec!['M', 'X', 'M', 'X', 'A', 'X', 'M', 'A', 'S', 'X'],
        ];
        let word = "XMAS";
        assert_eq!(count_occurrences(grid, word), 18);
    }

    // Auto-generated, yay!
    #[test]
    fn test_string_to_grid() {
        let input = "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        let expected = vec![
            vec!['M', 'M', 'M', 'S', 'X', 'X', 'M', 'A', 'S', 'M'],
            vec!['M', 'S', 'A', 'M', 'X', 'M', 'S', 'M', 'S', 'A'],
            vec!['A', 'M', 'X', 'S', 'X', 'M', 'A', 'A', 'M', 'M'],
            vec!['M', 'S', 'A', 'M', 'A', 'S', 'M', 'S', 'M', 'X'],
            vec!['X', 'M', 'A', 'S', 'A', 'M', 'X', 'A', 'M', 'M'],
            vec!['X', 'X', 'A', 'M', 'M', 'X', 'X', 'A', 'M', 'A'],
            vec!['S', 'M', 'S', 'M', 'S', 'A', 'S', 'X', 'S', 'S'],
            vec!['S', 'A', 'X', 'A', 'M', 'A', 'S', 'A', 'A', 'A'],
            vec!['M', 'A', 'M', 'M', 'M', 'X', 'M', 'M', 'M', 'M'],
            vec!['M', 'X', 'M', 'X', 'A', 'X', 'M', 'A', 'S', 'X'],
        ];
        assert_eq!(string_to_grid(input), expected);
    }
}
