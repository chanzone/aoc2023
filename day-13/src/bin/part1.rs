use std::cmp;

fn main() {
    // When done, run `cargo run --bin <folder name> to get the output`

    let input = include_str!("./input.txt");
    let output = process(input);
    dbg!(output);
}

fn get_middle_column_vertical(pattern: &Vec<Vec<char>>) -> i32 {
    let mut index: i32 = -1;

    for x in 0..pattern[0].len() - 1 {
        let mut is_potential_middle = true;
        for row in pattern {
            if row[x] != row[x+1] {
                is_potential_middle = false;
                break;
            }
        }

        if is_potential_middle {
            let mut is_middle = true;
            let examine_range = cmp::min(pattern[0].len() - 1 - x, x + 1);
            'outer: for i in 1..examine_range {
                for row in pattern {
                    if row[x - i] != row[x + 1 + i] {
                        is_middle = false;
                        break 'outer;
                    }
                }
            }

            if is_middle {
                index = x as i32;
                break;
            }
        }
    }

    index + 1
}

fn get_summarize(pattern: &Vec<Vec<char>>) -> i32 {
    let mut summarize = get_middle_row_horizontal(pattern);

    if summarize == 0 {
        summarize = get_middle_column_vertical(pattern);
    }

    summarize
}

fn get_middle_row_horizontal(pattern: &Vec<Vec<char>>) -> i32 {
    let mut index: i32 = -1;

    for y in 0..pattern.len() - 1 {
        let mut is_potential_middle = true;
        for x in 0..pattern[0].len() {
            if pattern[y][x] != pattern[y+1][x] {
                is_potential_middle = false;
                break;
            }
        }

        if is_potential_middle {
            let mut is_middle = true;
            let examine_range = cmp::min(pattern.len() - 1 - y, y + 1);
            'outer: for i in 1..examine_range {
                for x in 0..pattern[0].len() {
                    if pattern[y - i][x] != pattern[y + 1 + i][x] {
                        is_middle = false;
                        break 'outer;
                    }
                }
            }

            if is_middle {
                index = y as i32;
                break;
            }
        }
    }

    (index + 1) * 100
}

fn process(input: &str) -> String {
    let mut pattern: Vec<Vec<char>> = Vec::new();
    let mut total_summarize: i32 = 0;

    for line in input.lines() {
        let line = line.trim();

        if line.is_empty() {
            total_summarize += get_summarize(&pattern);
            pattern = Vec::new();
            continue;
        }

        pattern.push(line.chars().collect());
    }

    total_summarize += get_summarize(&pattern);

    total_summarize.to_string()   // Replace with the actual code here
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // For test, run `cargo test --bin <folder name> to verify the test output`

        let input = include_str!("./input-test.txt");
        let result = process(input);
        assert_eq!(result, "405".to_string()); // Replace with the expected test output here
    }
}