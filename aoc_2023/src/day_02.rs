use common::{Answer, Solution};

pub struct Day02;

// 12 red cubes, 13 green cubes, and 14 blue cubes
const MAX_CUBES: [u32; 3] = [12, 13, 14];

impl Solution for Day02 {
    fn name(&self) -> &'static str {
        "Cube Conundrum"
    }

    fn part_a(&self, input: &str) -> Answer {
        let runs = input
            .lines()
            .map(|line| {
                let (game, cubes) = line.split_once(':').unwrap();
                let game = game.trim_start_matches("Game ").parse::<u32>().unwrap();
                let sets = cubes.split(';').collect::<Vec<_>>();

                let mut cube_sets = Vec::new();
                for part in sets {
                    let iter = part.trim().split(',');

                    let mut cubes = CubeSet::default();
                    for i in iter {
                        let mut iter = i.trim().split_whitespace();
                        let count = iter.next().unwrap().parse::<u32>().unwrap();
                        let color = iter.next().unwrap();

                        match color {
                            "red" => cubes.red += count,
                            "green" => cubes.green += count,
                            "blue" => cubes.blue += count,
                            _ => unreachable!(),
                        }
                    }
                    cube_sets.push(cubes);
                }

                (game, cube_sets)
            })
            .filter(|(_, games)| {
                for game in games {
                    if game.red <= MAX_CUBES[0]
                        && game.green <= MAX_CUBES[1]
                        && game.blue <= MAX_CUBES[2]
                    {
                        continue;
                    } else {
                        return false;
                    }
                }
                true
            })
            .map(|x| x.0)
            .sum::<u32>();
        runs.into()
    }

    fn part_b(&self, input: &str) -> Answer {
        let runs = input
            .lines()
            .map(|line| {
                let (game, cubes) = line.split_once(':').unwrap();
                let game = game.trim_start_matches("Game ").parse::<u32>().unwrap();
                let sets = cubes.split(';').collect::<Vec<_>>();

                let mut cube_sets = Vec::new();
                for part in sets {
                    let iter = part.trim().split(',');

                    let mut cubes = CubeSet::default();
                    for i in iter {
                        let mut iter = i.trim().split_whitespace();
                        let count = iter.next().unwrap().parse::<u32>().unwrap();
                        let color = iter.next().unwrap();

                        match color {
                            "red" => cubes.red += count,
                            "green" => cubes.green += count,
                            "blue" => cubes.blue += count,
                            _ => unreachable!(),
                        }
                    }
                    cube_sets.push(cubes);
                }

                (game, cube_sets)
            })
            .map(|(num, games)| {
                let mut max = CubeSet::default();
                for game in games {
                    max = max.max(&game);
                }
                max.red * max.green * max.blue
            })
            .sum::<u32>();
        runs.into()
    }
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
}

#[cfg(test)]
mod test {
    use common::Solution;
    use indoc::indoc;

    use super::Day02;

    const CASE: &str = indoc! {"
        Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
    "};

    #[test]
    fn part_a() {
        assert_eq!(Day02.part_a(CASE), 8.into());
    }

    #[test]
    fn part_b() {
        assert_eq!(Day02.part_b(CASE), 2286.into());
    }
}
