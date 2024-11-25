use common::{Answer, Solution};

// 12 red cubes, 13 green cubes, and 14 blue cubes
const MAX_CUBES: [u32; 3] = [12, 13, 14];

pub const SOLUTION: Solution = Solution {
    name: "Cube Conundrum",
    date: (2023, 02),

    part_a,
    part_b,
};

fn part_a(input: &str) -> Answer {
    parse(input)
        .iter()
        .enumerate()
        .filter(|(_, games)| games.iter().all(|game| game.is_possible()))
        .map(|x| x.0 + 1)
        .sum::<usize>()
        .into()
}

fn part_b(input: &str) -> Answer {
    parse(input)
        .iter()
        .map(|games| {
            let mut max = CubeSet::default();
            for game in games {
                max = max.max(game);
            }
            max.red * max.green * max.blue
        })
        .sum::<u32>()
        .into()
}

fn parse(input: &str) -> Vec<Vec<CubeSet>> {
    input
        .lines()
        .map(|line| {
            let cubes = line.split_once(':').unwrap().1;

            let mut sets = Vec::new();
            for game in cubes.split(';') {
                let mut cubes = CubeSet::default();
                for i in game.split(',') {
                    let mut iter = i.split_whitespace();
                    let count = iter.next().unwrap().parse::<u32>().unwrap();
                    let color = iter.next().unwrap();

                    match color {
                        "red" => cubes.red += count,
                        "green" => cubes.green += count,
                        "blue" => cubes.blue += count,
                        _ => unreachable!(),
                    }
                }
                sets.push(cubes);
            }

            sets
        })
        .collect()
}

#[derive(Debug, Default)]
struct CubeSet {
    red: u32,
    green: u32,
    blue: u32,
}

impl CubeSet {
    fn max(&self, other: &Self) -> Self {
        Self {
            red: self.red.max(other.red),
            green: self.green.max(other.green),
            blue: self.blue.max(other.blue),
        }
    }

    fn is_possible(&self) -> bool {
        self.red <= MAX_CUBES[0] && self.green <= MAX_CUBES[1] && self.blue <= MAX_CUBES[2]
    }
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    const CASE: &str = indoc! {"
        Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
    "};

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(CASE), 8.into());
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(CASE), 2286.into());
    }
}
