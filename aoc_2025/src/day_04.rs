use aoc_lib::matrix::Grid;
use common::{Answer, solution};
use nd_vec::Vec2;

solution!("Printing Department", 4);

#[derive(Clone)]
enum Tile {
    Empty,
    Roll,
}

fn part_a(input: &str) -> Answer {
    let grid = Grid::parse(input, |chr| match chr {
        '.' => Tile::Empty,
        '@' => Tile::Roll,
        _ => panic!(),
    });

    count(&grid).0.into()
}

fn part_b(input: &str) -> Answer {
    let mut grid = Grid::parse(input, |chr| match chr {
        '.' => Tile::Empty,
        '@' => Tile::Roll,
        _ => panic!(),
    });

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

fn count(grid: &Grid<Tile>) -> (u32, Grid<Tile>) {
    let mut out = 0;

    let mut next = grid.clone();

    for y in 0..grid.size.y() {
        for x in 0..grid.size().x() {
            let pos = Vec2::new([x, y]).num_cast::<i32>().unwrap();
            if matches!(grid.get(pos.num_cast().unwrap()).unwrap(), Tile::Empty) {
                continue;
            }

            let mut neighbors = 0;
            for dy in -1..=1 {
                for dx in -1..=1 {
                    let delta = Vec2::new([dx, dy]);
                    let check = pos + delta;

                    if delta == Vec2::zero()
                        || check.x() < 0
                        || check.y() < 0
                        || check.x() >= grid.size.x() as i32
                        || check.y() >= grid.size.y() as i32
                    {
                        continue;
                    }

                    if matches!(grid.get(check.num_cast().unwrap()).unwrap(), Tile::Roll) {
                        neighbors += 1;
                    }
                }
            }

            if neighbors < 4 {
                out += 1;
                next.set(pos, Tile::Empty);
            }
        }
    }

    (out, next)
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
