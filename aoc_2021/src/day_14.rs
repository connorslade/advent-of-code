use hashbrown::HashMap;

use common::{solution, Answer};

solution!("Extended Polymerization", 14);

fn part_a(input: &str) -> Answer {
    process(input, 10).into()
}

fn part_b(input: &str) -> Answer {
    process(input, 40).into()
}

fn process(raw: &str, steps: usize) -> u64 {
    let counts = Polymer::parse(raw).process(steps);
    counts.iter().max().unwrap() - counts.iter().filter(|&&x| x != 0).min().unwrap()
}

#[derive(Debug)]
struct Polymer {
    units: Vec<char>,
    key: HashMap<[char; 2], char>,
}

impl Polymer {
    fn process(&mut self, steps: usize) -> [u64; 26] {
        fn index(unit: char) -> usize {
            unit as usize - 'A' as usize
        }

        let mut pairs = HashMap::<_, u64>::new();
        let mut counts = [0; 26];

        counts[index(*self.units.last().unwrap())] += 1;
        for units in self.units.windows(2) {
            counts[index(units[0])] += 1;
            *pairs.entry([units[0], units[1]]).or_default() += 1;
        }

        // AB -> C
        // (A, B) -> (A, C), (C, B)
        // C += 1
        for _ in 0..steps {
            let mut new_pairs = HashMap::new();

            for (pair, count) in pairs.iter() {
                let mapping = self.key[pair];

                *new_pairs.entry([pair[0], mapping]).or_default() += count;
                *new_pairs.entry([mapping, pair[1]]).or_default() += count;
                counts[index(mapping)] += count;
            }

            pairs = new_pairs;
        }

        counts
    }

    fn parse(raw: &str) -> Self {
        let (start, key) = raw.split_once("\n\n").unwrap();
        let mut key_out = HashMap::new();

        for i in key.lines() {
            let (k, v) = i.split_once(" -> ").unwrap();
            let mut k = k.chars();
            key_out.insert(
                [k.next().unwrap(), k.next().unwrap()],
                v.chars().next().unwrap(),
            );
        }

        Self {
            units: start.chars().collect(),
            key: key_out,
        }
    }
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    const CASE: &str = indoc! {"
        NNCB

        CH -> B
        HH -> N
        CB -> H
        NH -> C
        HB -> C
        HC -> B
        HN -> C
        NN -> C
        BH -> H
        NC -> B
        NB -> B
        BN -> B
        BB -> N
        BC -> B
        CC -> N
        CN -> C
    "};

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(CASE), 1588.into());
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(CASE), 2188189693529_u64.into());
    }
}
