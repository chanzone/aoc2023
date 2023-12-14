// NOTE: NOT WORKING. NO RESULT AFTER COMPILING FOR LONG
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

fn tilt(platform: &mut Vec<Vec<usize>>, is_reverse: bool) {
    for column in platform.iter_mut() {

        if is_reverse {
            column.reverse();
        }

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

        if is_reverse {
            column.reverse();
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
    let mut platform: Vec<Vec<usize>> = extract(input);
    let mut total_load: usize = 0;

    for _ in 1..1000000000 {
        platform = swap(platform);
        tilt(&mut platform, false);
        platform = swap(platform);
        tilt(&mut platform, false);
        platform = swap(platform);
        tilt(&mut platform, true);
        platform = swap(platform);
        tilt(&mut platform, true);
    }

    let platform = swap(platform);

    for column in platform {
        total_load += calculate_load(&column);
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
        assert_eq!(result, "64".to_string()); // Replace with the expected test output here
    }
}