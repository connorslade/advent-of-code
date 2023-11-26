use common::{Answer, Solution};

pub struct Day20;

impl Solution for Day20 {
    fn name(&self) -> &'static str {
        "Grove Positioning System"
    }

    fn part_a(&self, input: &str) -> Answer {
        let mut file = File::new(input);
        file.mix();
        file.coordinates().into()
    }

    fn part_b(&self, input: &str) -> Answer {
        let mut file = File::new(input).multiply(811589153);
        (0..10).for_each(|_| file.mix());
        file.coordinates().into()
    }
}

#[derive(Debug)]
struct File {
    // (value, index)
    list: Vec<(i64, usize)>,
}

impl File {
    fn new(raw: &str) -> Self {
        Self {
            list: raw
                .lines()
                .map(|x| x.parse().unwrap())
                .enumerate()
                .map(|(i, v)| (v, i))
                .collect(),
        }
    }

    fn coordinates(&self) -> i64 {
        let zero = self.list.iter().position(|x| x.0 == 0).unwrap() as isize;
        self.get(zero + 1000).unwrap()
            + self.get(zero + 2000).unwrap()
            + self.get(zero + 3000).unwrap()
    }

    fn multiply(self, val: i64) -> Self {
        Self {
            list: self.list.into_iter().map(|x| (x.0 * val, x.1)).collect(),
        }
    }

    fn get(&self, index: isize) -> Option<&i64> {
        self.list
            .get(index as usize % self.list.len())
            .map(|(v, _)| v)
    }

    fn mix(&mut self) {
        for i in 0..self.list.len() {
            let (index, value) = self
                .list
                .iter()
                .enumerate()
                .find(|x| x.1 .1 == i)
                .map(|x| (x.0, *x.1))
                .unwrap();

            self.list.remove(index);
            let raw_i = index as i64 + value.0;
            let new_i = if value.0 > 0 {
                raw_i % self.list.len() as i64
            } else if raw_i < 0 {
                self.list.len() as i64 - (raw_i.abs() % self.list.len() as i64)
            } else {
                raw_i
            };
            self.list.insert(new_i as usize, value);
        }
    }
}
