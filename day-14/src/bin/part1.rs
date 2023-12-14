fn main() {
    // When done, run `cargo run --bin <folder name> to get the output`

    let input = include_str!("./input.txt");
    let output = process(input);
    dbg!(output);
}

fn extract(input: &str) -> Vec<Vec<usize>> {
    input.lines()
        .map(|line| line.chars().map(|c| {
            if c == '.' {
                0
            } else if c == 'O' {
                1
            } else {
                9
            }
        }).collect())
        .collect()
}

#[allow(clippy::needless_range_loop)]
fn swap(platform: Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let mut columns: Vec<Vec<usize>> = vec![vec![0; platform[0].len()]; platform.len()];
    for x in 0..columns[0].len() {
        for y in 0..columns.len() {
            columns[x][y] = platform[y][x];
        }
    }

    columns
}

fn tilt(column: &mut Vec<usize>) {
    for i in 0..column.len() {
        match column[i] {
            0 => {
                for j in i+1..column.len() {
                    match column[j] {
                        0 => continue,
                        1 => {
                            column[i] = 1;
                            column[j] = 0;
                            break;
                        },
                        _ => break,
                    }
                }
            },
            9 => {
                column[i] = 0;
                continue;
            },
            _ => continue,
        }
    }
}

fn calculate_load(column: &Vec<usize>) -> usize {
    let mut load: usize = 0;

    for i in 0..column.len() {
        if column[i] == 1 {
            load += column.len() - i;
        }
    }

    load
}

fn process(input: &str) -> String {
    let platform: Vec<Vec<usize>> = extract(input);
    let mut columns: Vec<Vec<usize>> = swap(platform);
    let mut total_load: usize = 0;

    for column in columns.iter_mut() {
        tilt(column);
        total_load += calculate_load(column);
    }

    total_load.to_string()   // Replace with the actual code here
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // For test, run `cargo test --bin <folder name> to verify the test output`

        let input = include_str!("./input-test.txt");
        let result = process(input);
        assert_eq!(result, "136".to_string()); // Replace with the expected test output here
    }
}