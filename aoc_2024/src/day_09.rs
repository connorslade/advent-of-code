use common::{solution, Answer};
use itertools::Itertools;

solution!("Disk Fragmenter", 9);

fn part_a(input: &str) -> Answer {
    let mut problem = Problem::parse(input);
    problem.sort_blocks();
    problem.score_blocks().into()
}

fn part_b(input: &str) -> Answer {
    let mut problem = Problem::parse(input);
    problem.sort_files();
    problem.score_files().into()
}

#[derive(Debug)]
struct Problem {
    blocks: Vec<Option<u32>>,
    files: Vec<File>,
}

#[derive(Debug)]
struct File {
    pos: u32,
    size: u8,
    id: u32,
}

impl Problem {
    fn parse(input: &str) -> Self {
        let mut blocks = Vec::new();
        let mut files = Vec::new();

        let mut id = 0;
        let mut pos = 0;

        for (idx, chr) in input.trim().char_indices() {
            let num = chr.to_digit(10).unwrap() as u8;

            if idx % 2 == 1 {
                for _ in 0..num {
                    blocks.push(None);
                }
            } else {
                files.push(File { pos, size: num, id });
                for _ in 0..num {
                    blocks.push(Some(id));
                }
                id += 1;
            }

            pos += num as u32;
        }

        Self { blocks, files }
    }

    fn sort_blocks(&mut self) {
        loop {
            let empty = self.blocks.iter().position(|x| x.is_none()).unwrap();
            let last = self.blocks.iter().rposition(|x| x.is_some()).unwrap();

            if last > empty {
                self.blocks.swap(empty, last);
            } else {
                break;
            }
        }
    }

    fn sort_files(&mut self) {
        let max_id = self.files.last().unwrap().id;
        for id in (0..=max_id).rev() {
            let file_idx = self.files.iter().position(|x| x.id == id).unwrap();
            let file = &self.files[file_idx];

            let mut new_pos = None;
            for (a, b) in self.files.iter().tuple_windows() {
                let free = (b.pos) - (a.pos + a.size as u32);
                let pos = a.pos + a.size as u32;

                if pos > file.pos {
                    break;
                }

                if free >= file.size as u32 {
                    new_pos = Some(pos);
                    break;
                }
            }

            if let Some(new_pos) = new_pos {
                self.files[file_idx].pos = new_pos;
            }

            self.files.sort_by_key(|x| x.pos);
        }
    }

    fn score_blocks(&self) -> u64 {
        let mut sum = 0;

        for (idx, id) in self.blocks.iter().enumerate() {
            sum += idx as u64 * id.unwrap_or_default() as u64;
        }

        sum
    }

    fn score_files(&self) -> u64 {
        let mut sum = 0;
        let mut last = 0;
        let mut idx = 0;
        for x in &self.files {
            for _ in last..x.pos {
                idx += 1;
            }

            for _ in 0..x.size {
                sum += idx * x.id as u64;
                idx += 1;
            }

            last = x.pos + x.size as u32;
        }
        sum
    }
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    const CASE: &str = indoc! {"
        2333133121414131402
    "};

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(CASE), 1928.into());
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(CASE), 2858.into());
    }
}
