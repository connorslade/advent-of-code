use std::collections::HashMap;

use common::{solution, Answer};

solution!("Aplenty", 19);

fn part_a(input: &str) -> Answer {
    let (rules, shapes) = parse(input);
    let mut out = 0;

    for shape in shapes {
        let mut workflow = "in";

        loop {
            let current_workflow = rules.get(workflow).unwrap();
            for rule in current_workflow {
                match rule {
                    Rule::Comparison {
                        field,
                        comparison,
                        value,
                        destination,
                    } => {
                        let val = shape[*field];
                        if match comparison {
                            Comparison::LessThan => val < *value,
                            Comparison::GreaterThan => val > *value,
                        } {
                            workflow = destination;
                            break;
                        }
                    }
                    Rule::Default { destination } => {
                        workflow = destination;
                        break;
                    }
                }
            }

            if workflow == "A" {
                out += shape.iter().sum::<u32>();
                break;
            } else if workflow == "R" {
                break;
            }
        }
    }

    out.into()
}

fn part_b(input: &str) -> Answer {
    let (rules, _) = parse(input);

    let range = [(1, 4000); 4];
    solve_b(&rules, range, "in").into()
}

fn solve_b(rules: &HashMap<&str, Vec<Rule>>, mut range: [(u32, u32); 4], map: &str) -> u64 {
    let mut out = 0;

    let mut solve = |range: [(u32, u32); 4], destination: &str| {
        if destination == "A" {
            out += calc_size(&range);
        } else if destination != "R" {
            out += solve_b(rules, range, destination);
        }
    };

    for rule in rules.get(map).unwrap() {
        match rule {
            Rule::Comparison {
                field,
                comparison,
                value,
                destination,
            } => {
                let mut new_range = range;
                let val = &mut new_range[*field];
                let rng = &mut range[*field];

                match comparison {
                    Comparison::GreaterThan if val.1 > *value => {
                        val.0 = val.0.max(*value + 1);
                        rng.1 = rng.1.min(*value);
                    }
                    Comparison::LessThan if val.0 < *value => {
                        val.1 = val.1.min(*value - 1);
                        rng.0 = rng.0.max(*value);
                    }
                    _ => continue,
                }

                solve(new_range, destination);
            }
            Rule::Default { destination } => solve(range, destination),
        }
    }

    out
}

fn calc_size(ranges: &[(u32, u32); 4]) -> u64 {
    let mut out = 1;
    for range in ranges {
        out *= range.1 as u64 - range.0 as u64 + 1;
    }
    out
}

#[derive(Debug, Copy, Clone)]
enum Comparison {
    LessThan,
    GreaterThan,
}

#[derive(Debug, Clone)]
enum Rule<'a> {
    Comparison {
        field: usize,
        comparison: Comparison,
        value: u32,
        destination: &'a str,
    },
    Default {
        destination: &'a str,
    },
}

fn parse(input: &str) -> (HashMap<&str, Vec<Rule<'_>>>, Vec<[u32; 4]>) {
    let mut rules_out = HashMap::new();
    let mut shapes = Vec::new();

    let (rule, shape) = input.split_once("\n\n").unwrap();
    for rule in rule.lines() {
        let (name, rule) = rule.split_once('{').unwrap();

        let mut rules = Vec::new();
        for destination in rule[..rule.len() - 1].split(',') {
            let Some((comparison, destination)) = destination.split_once(':') else {
                rules.push(Rule::Default { destination });
                continue;
            };
            let comp = match &comparison[1..2] {
                "<" => Comparison::LessThan,
                ">" => Comparison::GreaterThan,
                _ => panic!("Invalid comparison"),
            };
            let value = comparison[2..].parse().unwrap();
            rules.push(Rule::Comparison {
                field: field_idx(&comparison[0..1]),
                comparison: comp,
                value,
                destination,
            });
        }

        rules_out.insert(name, rules);
    }

    for shape in shape.lines() {
        let mut x = [0; 4];
        for field in shape[1..shape.len() - 1].split(',') {
            let (field, value) = field.split_once('=').unwrap();
            let value = value.parse().unwrap();
            x[field_idx(field)] = value;
        }
        shapes.push(x);
    }

    (rules_out, shapes)
}

fn field_idx(field: &str) -> usize {
    match field {
        "x" => 0,
        "m" => 1,
        "a" => 2,
        "s" => 3,
        _ => panic!(),
    }
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    const CASE: &str = indoc! {"
        px{a<2006:qkq,m>2090:A,rfg}
        pv{a>1716:R,A}
        lnx{m>1548:A,A}
        rfg{s<537:gd,x>2440:R,A}
        qs{s>3448:A,lnx}
        qkq{x<1416:A,crn}
        crn{x>2662:A,R}
        in{s<1351:px,qqz}
        qqz{s>2770:qs,m<1801:hdj,R}
        gd{a>3333:R,R}
        hdj{m>838:A,pv}

        {x=787,m=2655,a=1222,s=2876}
        {x=1679,m=44,a=2067,s=496}
        {x=2036,m=264,a=79,s=2244}
        {x=2461,m=1339,a=466,s=291}
        {x=2127,m=1623,a=2188,s=1013}
    "};

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(CASE), 19114.into());
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(CASE), 167409079868000_u64.into());
    }
}
