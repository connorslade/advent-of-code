use aoc_lib::{direction::ordinal, matrix::Grid};
use common::{Answer, solution};

solution!("Printing Department", 4);

#[derive(Clone, PartialEq, Eq)]
enum Tile {
    Empty,
    Roll,
}

fn part_a(input: &str) -> Answer {
    let (count, _) = count(&parse(input));
    count.into()
}

fn part_b(input: &str) -> Answer {
    let mut grid = parse(input);
    let mut out = 0;

    loop {
        let (removed, next) = count(&grid);
        out += removed;
        grid = next;
        if removed == 0 {
            break;
        }
    }

    out.into()
}

fn parse(input: &str) -> Grid<Tile> {
    Grid::parse(input, |chr| match chr {
        '.' => Tile::Empty,
        '@' => Tile::Roll,
        _ => panic!(),
    })
}

fn count(grid: &Grid<Tile>) -> (u32, Grid<Tile>) {
    let mut next = grid.clone();
    let mut count = 0;

    for (pos, _tile) in grid.iter().filter(|x| *x.1 == Tile::Roll) {
        let mut neighbors = 0;
        for dir in ordinal::Direction::ALL {
            let tile = dir.try_advance(pos).and_then(|pos| grid.get(pos));
            neighbors += (tile == Some(&Tile::Roll)) as u32;
        }

        if neighbors < 4 {
            count += 1;
            next.set(pos, Tile::Empty);
        }
    }

    (count, next)
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    const CASE: &str = indoc! {"
        ..@@.@@@@.
        @@@.@.@.@@
        @@@@@.@.@@
        @.@@@@..@.
        @@.@@@@.@@
        .@@@@@@@.@
        .@.@.@.@@@
        @.@@@.@@@@
        .@@@@@@@@.
        @.@.@@@.@.
    "};

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(CASE), 13.into());
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(CASE), 43.into());
    }
}
