use aoc_lib::{direction::cardinal::Direction, matrix::Matrix};
use common::{solution, Answer};
use nd_vec::{vector, Vec2};

solution!("Warehouse Woes", 15);

fn part_a(input: &str) -> Answer {
    let mut problem = Problem::parse(input, false);
    problem.tick_all(false);
    problem.score().into()
}

fn part_b(input: &str) -> Answer {
    let mut problem = Problem::parse(input, true);
    problem.tick_all(true);
    problem.score().into()
}

struct Problem {
    pos: Vec2<usize>,
    idx: usize,

    board: Matrix<Tile>,
    instructions: Vec<Direction>,
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum Tile {
    Robot,
    Wall,
    Box,
    BoxRight,
    Empty,
}

impl Problem {
    fn parse(input: &str, part_b: bool) -> Self {
        let (board, instructions) = input.split_once("\n\n").unwrap();
        let mut board = Matrix::new_chars(board, |chr| match chr {
            '@' => Tile::Robot,
            '#' => Tile::Wall,
            'O' => Tile::Box,
            '.' => Tile::Empty,
            _ => panic!(),
        });

        // For part B, double the width of the board and convert the previously
        // one tile boxes into a Tile::Box and Tile::BoxRight
        if part_b {
            board.size = vector!(board.size.x() * 2, board.size.y());
            let mut i = 0;
            while i < board.data.len() {
                board.data.insert(
                    i + 1,
                    match board.data[i] {
                        Tile::Box => Tile::BoxRight,
                        Tile::Robot => Tile::Empty,
                        x => x,
                    },
                );
                i += 2;
            }
        }

        let instructions = instructions
            .chars()
            .filter_map(|x| {
                Some(match x {
                    '<' => Direction::Left,
                    '>' => Direction::Right,
                    '^' => Direction::Up,
                    'v' => Direction::Down,
                    _ => return None,
                })
            })
            .collect::<Vec<_>>();

        let pos = board.find(Tile::Robot).unwrap();
        board.set(pos, Tile::Empty);

        Self {
            pos,
            idx: 0,

            board,
            instructions,
        }
    }

    fn tick_all(&mut self, part_b: bool) {
        (0..self.instructions.len()).for_each(|_| self.tick(part_b));
    }

    fn tick(&mut self, part_b: bool) {
        let dir = self.instructions[self.idx];
        self.idx += 1;

        let new = dir.advance(self.pos);
        if {
            if part_b {
                self.push_b(new, dir)
            } else {
                self.push(new, dir)
            }
        } {
            self.pos = new;
        }
    }

    fn score(&self) -> u32 {
        self.board
            .iter()
            .filter(|x| *x.1 == Tile::Box)
            .map(|(pos, _)| (100 * pos.y() + pos.x()) as u32)
            .sum()
    }

    // -> was successful
    fn push(&mut self, pos: Vec2<usize>, dir: Direction) -> bool {
        // if we are air, return true
        let value = self.board[pos];
        match value {
            Tile::Empty => return true,
            Tile::Wall => return false,
            _ => {}
        }

        // if where we want to move is full, try to move that
        let new = dir.wrapping_advance(pos);
        if !self.board.contains(new) {
            return false;
        }

        if self.board[new] == Tile::Empty || self.push(new, dir) {
            self.board.set(new, value);
            self.board.set(pos, Tile::Empty);
            true
        } else {
            false
        }
    }

    // these next two function are an absolute disaster, but im too tired to
    // clean them up right now...

    fn can_push(&self, pos: Vec2<usize>, dir: Direction) -> bool {
        // println!("{pos:?}, {dir:?}");
        let value = self.board[pos];
        match value {
            Tile::Empty => return true,
            Tile::Wall => return false,
            Tile::Box | Tile::BoxRight => {}
            Tile::Robot => unreachable!(),
        }

        let other_box = match value {
            Tile::Box => pos + vector!(1, 0),
            Tile::BoxRight => pos - vector!(1, 0),
            _ => unreachable!(),
        };

        // if where we want to move is full, try to move that
        let new_a = dir.wrapping_advance(pos);
        let new_b = dir.wrapping_advance(other_box);
        if !(self.board.contains(new_a) && self.board.contains(new_b)) {
            return false;
        }

        (self.board[new_a] == Tile::Empty && self.board[new_b] == Tile::Empty)
            || ((new_a == other_box || self.can_push(new_a, dir))
                && (new_b == pos || self.can_push(new_b, dir)))
    }

    fn push_b(&mut self, pos: Vec2<usize>, dir: Direction) -> bool {
        if self.can_push(pos, dir) {
            let value = self.board[pos];
            if value == Tile::Empty {
                return true;
            }

            assert!(matches!(value, Tile::Box | Tile::BoxRight));
            let other_box = match value {
                Tile::Box => pos + vector!(1, 0),
                Tile::BoxRight => pos - vector!(1, 0),
                _ => unreachable!(),
            };
            let other_value = self.board[other_box];

            let new_a = dir.wrapping_advance(pos);
            let new_b = dir.wrapping_advance(other_box);
            if !(self.board.contains(new_a) && self.board.contains(new_b)) {
                return false;
            }

            if (self.board[new_a] == Tile::Empty && self.board[new_b] == Tile::Empty)
                || ((new_a == other_box || self.push_b(new_a, dir))
                    && (new_b == pos || self.push_b(new_b, dir)))
            {
                // do push
                self.board.set(new_a, value);
                self.board.set(pos, Tile::Empty);

                self.board.set(new_b, other_value);
                if other_box != new_a {
                    self.board.set(other_box, Tile::Empty);
                }
                true
            } else {
                false
            }
        } else {
            false
        }
    }
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    const CASE: &str = indoc! {"
        ##########
        #..O..O.O#
        #......O.#
        #.OO..O.O#
        #..O@..O.#
        #O#..O...#
        #O..O..O.#
        #.OO.O.OO#
        #....O...#
        ##########

        <vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
        vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
        ><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
        <<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
        ^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
        ^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
        >^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
        <><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
        ^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
        v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
    "};

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(CASE), 10092.into());
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(CASE), 9021.into());
    }
}
