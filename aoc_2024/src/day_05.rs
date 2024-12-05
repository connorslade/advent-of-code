use std::collections::{HashMap, HashSet};

use common::{solution, Answer};

solution!("Print Queue", 5);

fn part_a(input: &str) -> Answer {
    let problem = PrintQueue::parse(input);

    // For each list of pages in an update, sum the middle value of all valid
    // ones.
    (0..problem.updates.len())
        .filter(|&x| problem.is_valid(x))
        .map(|x| &problem.updates[x])
        .map(|x| x[x.len() / 2])
        .sum::<u32>()
        .into()
}

fn part_b(input: &str) -> Answer {
    let problem = PrintQueue::parse(input);

    // For each list of pages that are not correctly sorted, sort them then find
    // the middle.
    (0..problem.updates.len())
        .filter(|&x| !problem.is_valid(x))
        .map(|x| problem.sort_pages(x))
        .map(|x| x[x.len() / 2])
        .sum::<u32>()
        .into()
}

struct PrintQueue {
    /// Maps a page to the pages that must come before it.
    rule_map: HashMap<u32, HashSet<u32>>,
    updates: Vec<Vec<u32>>,
}

impl PrintQueue {
    fn parse(input: &str) -> Self {
        let (rules, updates) = input.split_once("\n\n").unwrap();

        // a|b => a comes before b
        // For each rule stating that some page a comes before some page b, add
        // a to the list of pages that come before b.
        let mut rule_map: HashMap<u32, HashSet<u32>> = HashMap::new();
        for (a, b) in rules.lines().map(|x| {
            let (a, b) = x.split_once('|').unwrap();
            (a.parse::<u32>().unwrap(), b.parse::<u32>().unwrap())
        }) {
            rule_map.entry(b).or_default().insert(a);
        }

        let updates = updates
            .lines()
            .map(|x| x.split(',').map(|x| x.parse::<u32>().unwrap()).collect())
            .collect();

        Self { rule_map, updates }
    }

    fn is_valid(&self, idx: usize) -> bool {
        let line = &self.updates[idx];
        // A line is sorted if to the left of every page is a page that should be before it.
        line.is_sorted_by(|a, b| self.rule_map.contains_key(b) && self.rule_map[b].contains(a))
    }

    fn sort_pages(&self, idx: usize) -> Vec<u32> {
        let mut line = self.updates[idx].clone();
        // Just the same expression from before used by a sorting algo to put our pages in order!
        line.sort_by(|a, b| {
            (self.rule_map.contains_key(b) && self.rule_map[b].contains(a)).cmp(&true)
        });
        line
    }
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    const CASE: &str = indoc! {"
        47|53
        97|13
        97|61
        97|47
        75|29
        61|13
        75|53
        29|13
        97|29
        53|29
        61|53
        97|53
        61|29
        47|13
        75|47
        97|75
        47|61
        75|61
        47|29
        75|13
        53|13

        75,47,61,53,29
        97,61,53,29,13
        75,29,13
        75,97,47,61,53
        61,13,29
        97,13,75,29,47
    "};

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(CASE), 143.into());
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(CASE), 123.into());
    }
}
