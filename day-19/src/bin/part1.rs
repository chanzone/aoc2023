use std::collections::HashMap;

type Rules = Vec<String>;

fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    dbg!(output);
}

fn get_result(
    workflows: &HashMap<String, Rules>,
    ratings: &HashMap<String, usize>,
    action: String,
) -> Option<bool> {
    match action.as_str() {
        "A" => Some(true),
        "R" => Some(false),
        _ => is_accepted(workflows, ratings, &action),
    }
}

fn is_accepted(
    workflows: &HashMap<String, Rules>,
    ratings: &HashMap<String, usize>,
    rule_name: &str,
) -> Option<bool> {
    let rules = workflows.get(rule_name).unwrap();

    let mut result: Option<bool> = None;

    for rule in rules.iter() {
        match rule.as_str() {
            "A" => {
                result = Some(true);
                break;
            }
            "R" => {
                result = Some(false);
                break;
            }
            _ => {
                let condition: &str;
                if rule.contains('>') {
                    condition = ">";
                } else if rule.contains('<') {
                    condition = "<";
                } else {
                    result = is_accepted(workflows, ratings, rule);
                    break;
                }

                let part_one = &mut rule.split(condition);
                let category: String = part_one.next().unwrap().to_string();
                let part_two = &mut part_one.next().unwrap().split(':');
                let rating_requirement: usize = part_two.next().unwrap().parse().unwrap();
                let action: String = part_two.next().unwrap().to_string();
                let rating: usize = *ratings.get(&category).unwrap();

                match condition {
                    "<" => {
                        if rating < rating_requirement {
                            result = get_result(workflows, ratings, action);
                        }
                    }
                    ">" => {
                        if rating > rating_requirement {
                            result = get_result(workflows, ratings, action);
                        }
                    }
                    _ => panic!("Invalid condition"),
                }
            }
        }

        if result.is_some() {
            break;
        }
    }

    result
}

fn process(input: &str) -> String {
    let rules_input = input
        .split("\n\n")
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    let workflows: HashMap<String, Rules> = rules_input[0]
        .lines()
        .map(|s| {
            let parts = &mut s[0..s.len() - 1].split('{');
            let name: String = parts.next().unwrap().to_string();
            let rules: Rules = parts
                .next()
                .unwrap()
                .split(',')
                .map(|s| s.to_string())
                .collect::<Vec<String>>();

            (name, rules)
        })
        .collect();

    rules_input[1].lines().fold(0, |acc, s| {
        let ratings: HashMap<String, usize> = s[1..s.len() - 1]
            .split(',')
            .map(|c| {
                let parts = &mut c.split('=');
                let category: String = parts.next().unwrap().to_string();
                let rating: usize = parts.next().unwrap().parse().unwrap();

                (category, rating)
            })
            .collect();

        if is_accepted(&workflows, &ratings, "in").unwrap() {
            acc + ratings.values().sum::<usize>()
        } else {
            acc
        }
    }).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./input-test.txt");
        let result = process(input);
        assert_eq!(result, "19114".to_string());
    }
}
