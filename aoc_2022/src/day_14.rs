use common::{Answer, Solution};
use nd_vec::vector;

type Point = nd_vec::Vec2<usize>;

const NEW_POINT: Point = vector!(500, 0);
pub struct Day14;

impl Solution for Day14 {
    fn name(&self) -> &'static str {
        "Regolith Reservoir"
    }

    fn part_a(&self, input: &str) -> Answer {
        let mut world = World::parse(input);

        'o: loop {
            world.working = NEW_POINT;
            *world.get_mut(NEW_POINT.x(), NEW_POINT.y()) = Element::Sand;

            while world.tick(false) {
                if world.working.y() >= world.bounds {
                    break 'o;
                }
            }
        }

        (world.count_sand() - 1).into()
    }

    fn part_b(&self, input: &str) -> Answer {
        let mut world = World::parse(input);

        loop {
            if world.working.y() == 0 {
                break;
            }

            world.working = NEW_POINT;
            *world.get_mut(NEW_POINT.x(), NEW_POINT.y()) = Element::Sand;
            while world.tick(true) {}
        }

        world.count_sand().into()
    }
}

#[derive(Debug)]
struct World {
    data: Matrix<Element>,
    working: Point,

    x_adjust: usize,
    y_max: usize,
    bounds: usize,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
enum Element {
    Wall,
    Sand,
    #[default]
    Air,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Matrix<T> {
    data: Vec<T>,
    width: usize,
}

#[derive(Debug)]
struct Line(Point, Point);

impl World {
    fn parse(raw: &str) -> Self {
        let mut lines = Vec::new();
        for i in raw.lines() {
            for j in i.split(" -> ").collect::<Vec<_>>().windows(2) {
                lines.push(Line::parse(j[0], j[1]));
            }
        }

        let (mut min_x, mut max_x, mut max_y) = (usize::MAX, 0, 0);
        for i in lines.iter() {
            min_x = min_x.min(i.0.x()).min(i.1.x());
            max_x = max_x.max(i.0.x()).max(i.1.x());
            max_y = max_y.max(i.0.y()).max(i.1.y());
        }

        let x_adjust = min_x - max_y;
        let mut out = Matrix::new_filled(max_x - min_x + max_x + 1, max_y + 3, Element::Air);
        lines
            .iter()
            .flat_map(|x| x.points())
            .for_each(|x| out.set(x.x() - x_adjust, x.y(), Element::Wall));

        Self {
            data: out,
            bounds: max_y,

            x_adjust,
            y_max: max_y,
            working: vector!(1, 1),
        }
    }

    fn tick(&mut self, floor: bool) -> bool {
        let (x, y) = (self.working.x(), self.working.y());

        for i in [(0_isize, 1), (-1, 1), (1, 1)] {
            let (nx, ny) = ((x as isize + i.0) as usize, y + i.1);
            if (floor && ny >= self.y_max + 2) || self.get(nx, ny) != &Element::Air {
                continue;
            }

            *self.get_mut(x, y) = Element::Air;
            *self.get_mut(nx, ny) = Element::Sand;
            self.working = vector!(nx, ny);
            return true;
        }

        false
    }

    fn count_sand(&self) -> usize {
        self.data
            .raw()
            .iter()
            .filter(|x| **x == Element::Sand)
            .count()
    }

    fn get(&self, x: usize, y: usize) -> &Element {
        self.data.get(x - self.x_adjust, y)
    }

    fn get_mut(&mut self, x: usize, y: usize) -> &mut Element {
        self.data.get_mut(x - self.x_adjust, y)
    }
}

impl<T> Matrix<T> {
    fn new_filled(width: usize, height: usize, val: T) -> Self
    where
        T: Clone,
    {
        Self {
            data: vec![val; width * height],
            width,
        }
    }

    fn raw(&self) -> &Vec<T> {
        &self.data
    }

    fn set(&mut self, x: usize, y: usize, data: T) {
        self.data[y * self.width + x] = data;
    }

    fn get(&self, x: usize, y: usize) -> &T {
        debug_assert!(x < self.width, "x out of bounds");
        &self.data[y * self.width + x]
    }

    fn get_mut(&mut self, x: usize, y: usize) -> &mut T {
        debug_assert!(x < self.width, "x out of bounds");
        &mut self.data[y * self.width + x]
    }
}

impl Line {
    fn parse(a: &str, b: &str) -> Self {
        fn point(x: &str) -> Point {
            let mut x = x.split(',').map(|x| x.parse::<usize>().unwrap());
            vector!(x.next().unwrap(), x.next().unwrap())
        }

        Self(point(a), point(b))
    }

    fn points(&self) -> Vec<Point> {
        let order_range = |a: usize, b: usize| a.min(b)..=a.max(b);
        let mut out = Vec::new();

        if self.0.x() == self.1.x() {
            for y in order_range(self.0.y(), self.1.y()) {
                out.push(vector!(self.0.x(), y));
            }
            return out;
        }

        for x in order_range(self.0.x(), self.1.x()) {
            out.push(vector!(x, self.0.y()));
        }
        out
    }
}
