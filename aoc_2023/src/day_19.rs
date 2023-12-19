use std::collections::HashMap;

use common::{Answer, Solution};

pub struct Day19;

impl Solution for Day19 {
    fn name(&self) -> &'static str {
        "Aplenty"
    }

    fn part_a(&self, input: &str) -> Answer {
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
                            let val = match field {
                                Field::X => shape.x,
                                Field::M => shape.m,
                                Field::A => shape.a,
                                Field::S => shape.s,
                            };
                            let result = match comparison {
                                Comparison::LessThan => val < *value,
                                Comparison::GreaterThan => val > *value,
                            };
                            if result {
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

                match workflow {
                    "A" => {
                        out += shape.x + shape.m + shape.a + shape.s;
                        break;
                    }
                    "R" => {
                        break;
                    }
                    _ => {}
                }
            }
        }

        out.into()
    }

    fn part_b(&self, input: &str) -> Answer {
        let (rules, _) = parse(input);

        let range = ShapeRange {
            x: (1, 4000),
            m: (1, 4000),
            a: (1, 4000),
            s: (1, 4000),
        };
        solve_b(&rules, range, "in").into()
    }
}

fn sz(ranges: &ShapeRange) -> i64 {
    let mut out = 1;
    out *= ranges.x.1 as i64 - ranges.x.0 as i64 + 1;
    out *= ranges.m.1 as i64 - ranges.m.0 as i64 + 1;
    out *= ranges.a.1 as i64 - ranges.a.0 as i64 + 1;
    out *= ranges.s.1 as i64 - ranges.s.0 as i64 + 1;
    out
}

fn solve_b(rules: &HashMap<String, Vec<Rule>>, mut range: ShapeRange, map: &str) -> i64 {
    let mut out = 0;

    for rule in rules.get(map).unwrap() {
        match rule {
            Rule::Comparison {
                field,
                comparison,
                value,
                destination,
            } => match comparison {
                Comparison::GreaterThan => {
                    let mut new_range = range.clone();
                    let val = match field {
                        Field::X => &mut new_range.x,
                        Field::M => &mut new_range.m,
                        Field::A => &mut new_range.a,
                        Field::S => &mut new_range.s,
                    };
                    if val.1 > *value {
                        val.0 = val.0.max(*value + 1);
                        if destination == "A" {
                            out += sz(&new_range);
                        } else if destination != "R" {
                            out += solve_b(rules, new_range, destination);
                        }

                        let val2 = match field {
                            Field::X => &mut range.x,
                            Field::M => &mut range.m,
                            Field::A => &mut range.a,
                            Field::S => &mut range.s,
                        };
                        val2.1 = val2.1.min(*value);
                    }
                }
                Comparison::LessThan => {
                    let mut new_range = range.clone();
                    let val = match field {
                        Field::X => &mut new_range.x,
                        Field::M => &mut new_range.m,
                        Field::A => &mut new_range.a,
                        Field::S => &mut new_range.s,
                    };
                    if val.0 < *value {
                        val.1 = val.1.min(*value - 1);
                        if destination == "A" {
                            out += sz(&new_range);
                        } else if destination != "R" {
                            out += solve_b(rules, new_range, destination);
                        }

                        let val2 = match field {
                            Field::X => &mut range.x,
                            Field::M => &mut range.m,
                            Field::A => &mut range.a,
                            Field::S => &mut range.s,
                        };
                        val2.0 = val2.0.max(*value);
                    }
                }
            },
            Rule::Default { destination } => {
                if destination == "A" {
                    out += sz(&range);
                } else if destination != "R" {
                    out += solve_b(rules, range, destination);
                }
            }
        }
    }

    out
}

#[derive(Debug, Copy, Clone)]
enum Comparison {
    LessThan,
    GreaterThan,
}

#[derive(Debug, Copy, Clone)]
enum Field {
    X,
    M,
    A,
    S,
}

#[derive(Debug, Clone)]
enum Rule {
    Comparison {
        field: Field,
        comparison: Comparison,
        value: u32,
        destination: String,
    },
    Default {
        destination: String,
    },
}

#[derive(Default, Debug, Copy, Clone)]
struct Shape {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}

#[derive(Default, Debug, Copy, Clone)]
struct ShapeRange {
    x: (u32, u32),
    m: (u32, u32),
    a: (u32, u32),
    s: (u32, u32),
}

fn parse(input: &str) -> (HashMap<String, Vec<Rule>>, Vec<Shape>) {
    let mut rules_out = HashMap::new();
    let mut shapes = Vec::new();

    let (rule, shape) = input.split_once("\n\n").unwrap();
    for rule in rule.lines() {
        // px{a<2006:qkq,m>2090:A,rfg}
        let (name, rule) = rule.split_once('{').unwrap();
        let rule = rule.split_once('}').unwrap().0;

        let mut rules = Vec::new();
        for i in rule.split(',') {
            if !i.contains(':') {
                rules.push(Rule::Default {
                    destination: i.to_string(),
                });
                continue;
            }

            let (comparison, destination) = i.split_once(':').unwrap();
            let field = match &comparison[0..1] {
                "x" => Field::X,
                "m" => Field::M,
                "a" => Field::A,
                "s" => Field::S,
                _ => panic!("Invalid field"),
            };
            let comp = match &comparison[1..2] {
                "<" => Comparison::LessThan,
                ">" => Comparison::GreaterThan,
                _ => panic!("Invalid comparison"),
            };
            let value = comparison[2..].parse().unwrap();
            rules.push(Rule::Comparison {
                field,
                comparison: comp,
                value,
                destination: destination.to_string(),
            });
        }

        rules_out.insert(name.to_string(), rules);
    }

    for shape in shape.lines() {
        let shape = &shape[1..shape.len() - 1];
        let mut x = Shape::default();
        for field in shape.split(',') {
            let (field, value) = field.split_once('=').unwrap();
            let value = value.parse().unwrap();
            match field {
                "x" => x.x = value,
                "m" => x.m = value,
                "a" => x.a = value,
                "s" => x.s = value,
                _ => panic!("Invalid field"),
            }
        }
        shapes.push(x);
    }

    (rules_out, shapes)
}

#[cfg(test)]
mod test {
    use common::Solution;
    use indoc::indoc;

    use super::Day19;

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
        assert_eq!(Day19.part_a(CASE), 19114.into());
    }

    #[test]
    fn part_b() {
        assert_eq!(Day19.part_b(CASE), 167409079868000_u64.into());
    }
}
