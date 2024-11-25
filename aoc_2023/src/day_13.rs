use common::{solution, Answer};

solution!("Point of Incidence", (2023, 04));

fn part_a(input: &str) -> Answer {
    let valleys = parse(input);
    solve(&valleys, 0).into()
}

fn part_b(input: &str) -> Answer {
    let valleys = parse(input);
    solve(&valleys, 1).into()
}

fn solve(valleys: &[Valley], limit: usize) -> usize {
    valleys
        .iter()
        .filter_map(|valley| {
            valley
                .horizontal_reflection(limit)
                .map(|x| 100 * x)
                .or_else(|| valley.vertical_reflection(limit))
        })
        .sum()
}

struct Valley {
    tiles: Vec<Vec<char>>,
}

fn parse(input: &str) -> Vec<Valley> {
    let mut out = Vec::new();

    for set in input.split("\n\n") {
        let tiles = set.lines().map(|line| line.chars().collect()).collect();
        out.push(Valley { tiles });
    }

    out
}

impl Valley {
    // Find a horizontal reflection in the valley.
    // Horizontal reflections is from left to right.
    fn horizontal_reflection(&self, error: usize) -> Option<usize> {
        for mid in 1..=self.tiles.len() - 1 {
            let side_len = mid.min(self.tiles.len() - mid);
            let start = mid - side_len;

            let mut diff = 0;
            for a in start..mid {
                let b = mid * 2 - a - 1;
                diff += (0..self.tiles[a].len())
                    .filter(|&i| self.tiles[a][i] != self.tiles[b][i])
                    .count();
            }

            if diff == error {
                return Some(mid);
            }
        }

        None
    }

    fn vertical_reflection(&self, error: usize) -> Option<usize> {
        for mid in 1..=self.tiles[0].len() - 1 {
            let side_len = mid.min(self.tiles[0].len() - mid);
            let start = mid - side_len;

            let mut diff = 0;
            for a in start..mid {
                let b = mid * 2 - a - 1;
                diff += (0..self.tiles.len())
                    .filter(|&i| self.tiles[i][a] != self.tiles[i][b])
                    .count();
            }

            if diff == error {
                return Some(mid);
            }
        }

        None
    }
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    const CASE: &str = indoc! {"
        #.##..##.
        ..#.##.#.
        ##......#
        ##......#
        ..#.##.#.
        ..##..##.
        #.#.##.#.

        #...##..#
        #....#..#
        ..##..###
        #####.##.
        #####.##.
        ..##..###
        #....#..#
    "};

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(CASE), 405.into());
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(CASE), 400.into());
    }
}
