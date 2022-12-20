use crate::{problem, Solution};

pub struct Day20;

impl Solution for Day20 {
    fn name(&self) -> &'static str {
        ""
    }

    fn part_a(&self) -> String {
        let raw = problem::load(2022, 20);
        let mut list = parse(&raw);

        for i in list.list.clone() {
            list.move_item(i.1);
            dbg!(&list);
        }

        dbg!(&list);
        todo!()
    }

    fn part_b(&self) -> String {
        let raw = problem::load(2022, 20);
        todo!()
    }
}

#[derive(Debug)]
struct WrapingList {
    // (value, innitial index)
    list: Vec<(i32, usize)>,
}

impl WrapingList {
    fn new(list: Vec<i32>) -> Self {
        Self {
            list: list.into_iter().enumerate().map(|(i, v)| (v, i)).collect(),
        }
    }

    fn get(&self, index: isize) -> Option<&i32> {
        if index < 0 {
            return self
                .list
                .get(self.list.len() - index.abs() as usize)
                .map(|(v, _)| v);
        }
        self.list.get(index as usize).map(|(v, _)| v)
    }

    fn move_item(&mut self, index: usize) {
        let (index, _) = self
            .list
            .iter()
            .enumerate()
            .find(|i| i.1 .1 == index)
            .unwrap();
        let (value, index) = self.list.remove(index);
        let new_index = index as i32 + value;

        let new_index = if new_index >= self.list.len() as i32 {
            new_index - self.list.len() as i32
        } else {
            new_index
        };

        self.list.insert(new_index as usize, (value, index));
    }
}

fn parse(raw: &str) -> WrapingList {
    let mut nums = Vec::new();

    for i in raw.lines() {
        nums.push(i.parse().unwrap());
    }

    WrapingList::new(nums)
}
