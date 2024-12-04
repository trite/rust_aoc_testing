// Part 1 took ~15 minutes
// No prompt for this one
pub fn part_1(input: &str) -> i32 {
    let grid = string_to_grid(input);
    count_occurrences(grid, "XMAS")
}

// Part 2 took ~30 minutes
/* I wrote the first version of this, which did not handle rotating the pattern yet.
   Prompted to have the function converted to handle rotating and running multiple times
   Prompt:
    Modify the part_2 function so that it performs the same action 4 times, using the rotate_grid function to rotate the pattern variable each time in between, and then summing the results.
*/
pub fn part_2(input: &str) -> i32 {
    let grid = string_to_grid(input);
    let mut pattern = vec![
        vec!['M', '.', 'S'],
        vec!['.', 'A', '.'],
        vec!['M', '.', 'S'],
    ];
    let mut total_count = 0;

    for _ in 0..4 {
        let subgrids = extract_subgrids(grid.clone(), 3);
        total_count += subgrids
            .iter()
            .filter(|subgrid| compare_with_wildcards(subgrid, &pattern))
            .count() as i32;
        pattern = rotate_grid(pattern);
    }

    total_count
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

/* Prompt:
   Create a function that takes a 2d vector of char as its first input, and an integer as its second. The function should return a list of every 2d vector that could exist within the first one whose dimensions in both x and y match the second input.

   So if provided this as the first input:

   ABCD
   EFGH
   IJKL
   MNOP

   And this as the second:

   3

   The result would be a vector returning 2d grids representing:

   ABC
   EFG
   IJK

   BCD
   FGH
   JKL

   EFG
   IJK
   MNO

   FGH
   JKL
   NOP
*/
fn extract_subgrids(
    grid: Vec<Vec<char>>,
    size: usize,
) -> Vec<Vec<Vec<char>>> {
    let mut subgrids = Vec::new();
    let rows = grid.len();
    let cols = grid[0].len();

    for i in 0..=rows.saturating_sub(size) {
        for j in 0..=cols.saturating_sub(size) {
            let mut subgrid = Vec::new();
            for x in 0..size {
                let mut row = Vec::new();
                for y in 0..size {
                    row.push(grid[i + x][j + y]);
                }
                subgrid.push(row);
            }
            subgrids.push(subgrid);
        }
    }
    subgrids
}

/* Prompt:
   Create a function that can compare 2 2d vectors of chars. The second one can contain dots (.), and each dot represents a wildcard character. The function should return true if every character in both vectors matches, where any character that is a dot is always a match.
*/
fn compare_with_wildcards(
    grid: &Vec<Vec<char>>,
    pattern: &Vec<Vec<char>>,
) -> bool {
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if pattern[i][j] != '.' && pattern[i][j] != grid[i][j] {
                return false;
            }
        }
    }
    true
}

/* Prompt:
   Create a function that can rotate a 2d vector of chars. If the input starts as:

   M.S
   .A.
   M.S

   Then it should become:

   S.S
   .A.
   M.M
*/
fn rotate_grid(grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let size = grid.len();
    let mut rotated = vec![vec![' '; size]; size];

    for i in 0..size {
        for j in 0..size {
            rotated[j][size - 1 - i] = grid[i][j];
        }
    }
    rotated
}

generate_tests!(
    2024,
    4,
    part_1,
    part_2,
    vec![("MMMSXXMASM\nMSAMXMSMSA\nAMXSXMAAMM\nMSAMASMSMX\nXMASAMXAMM\nXXAMMXXAMA\nSMSMSASXSS\nSAXAMASAAA\nMAMMMXMMMM\nMXMXAXMASX", 18)], // part 1 examples
    vec![("MMMSXXMASM\nMSAMXMSMSA\nAMXSXMAAMM\nMSAMASMSMX\nXMASAMXAMM\nXXAMMXXAMA\nSMSMSASXSS\nSAXAMASAAA\nMAMMMXMMMM\nMXMXAXMASX", 9)],         // part 2 examples
    Some(2618),
    Some(2011)
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

    // All remaining tests auto-generated, yay!
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

    #[test]
    fn test_extract_subgrids() {
        let grid = vec![
            vec!['A', 'B', 'C', 'D'],
            vec!['E', 'F', 'G', 'H'],
            vec!['I', 'J', 'K', 'L'],
            vec!['M', 'N', 'O', 'P'],
        ];
        let size = 3;
        let expected = vec![
            vec![
                vec!['A', 'B', 'C'],
                vec!['E', 'F', 'G'],
                vec!['I', 'J', 'K'],
            ],
            vec![
                vec!['B', 'C', 'D'],
                vec!['F', 'G', 'H'],
                vec!['J', 'K', 'L'],
            ],
            vec![
                vec!['E', 'F', 'G'],
                vec!['I', 'J', 'K'],
                vec!['M', 'N', 'O'],
            ],
            vec![
                vec!['F', 'G', 'H'],
                vec!['J', 'K', 'L'],
                vec!['N', 'O', 'P'],
            ],
        ];
        assert_eq!(extract_subgrids(grid, size), expected);
    }

    #[test]
    fn test_compare_with_wildcards() {
        let grid = vec![
            vec!['A', 'B', 'C'],
            vec!['D', 'E', 'F'],
            vec!['G', 'H', 'I'],
        ];
        let pattern = vec![
            vec!['A', '.', 'C'],
            vec!['.', 'E', '.'],
            vec!['G', '.', 'I'],
        ];
        assert!(compare_with_wildcards(&grid, &pattern));

        let pattern = vec![
            vec!['A', '.', 'C'],
            vec!['.', 'X', '.'],
            vec!['G', '.', 'I'],
        ];
        assert!(!compare_with_wildcards(&grid, &pattern));
    }

    #[test]
    fn test_rotate_grid() {
        let grid = vec![
            vec!['M', '.', 'S'],
            vec!['.', 'A', '.'],
            vec!['M', '.', 'S'],
        ];
        let expected = vec![
            vec!['M', '.', 'M'],
            vec!['.', 'A', '.'],
            vec!['S', '.', 'S'],
        ];
        assert_eq!(rotate_grid(grid), expected);
    }
}
