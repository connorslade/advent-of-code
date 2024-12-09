use common::{solution, Answer};
use itertools::{repeat_n, Itertools};

solution!("Disk Fragmenter", 9);

fn part_a(input: &str) -> Answer {
    let mut problem = Blocks::parse(input);
    problem.sort();
    problem.score().into()
}

fn part_b(input: &str) -> Answer {
    let mut problem = Files::parse(input);
    problem.sort();
    problem.score().into()
}

struct Blocks {
    blocks: Vec<Option<u32>>,
}

struct Files {
    files: Vec<File>,
}

struct File {
    pos: u32,
    size: u8,
    id: u32,
}

impl Blocks {
    fn parse(input: &str) -> Self {
        let mut blocks = Vec::new();

        let mut id = 0;
        for (idx, chr) in input.trim().char_indices() {
            let count = chr.to_digit(10).unwrap() as u8;

            let is_block = idx % 2 == 0;
            let item = is_block.then_some(id);

            blocks.extend(repeat_n(item, count as usize));
            id += is_block as u32;
        }

        Self { blocks }
    }

    fn sort(&mut self) {
        loop {
            let empty = self.blocks.iter().position(|x| x.is_none()).unwrap();
            let last = self.blocks.iter().rposition(|x| x.is_some()).unwrap();

            if last <= empty {
                break;
            }

            self.blocks.swap(empty, last);
        }
    }

    fn score(&self) -> u64 {
        self.blocks
            .iter()
            .enumerate()
            .map(|(idx, id)| idx as u64 * id.unwrap_or_default() as u64)
            .sum()
    }
}

impl Files {
    fn parse(input: &str) -> Self {
        let mut files = Vec::new();
        let (mut id, mut pos) = (0, 0);

        for (idx, chr) in input.trim().char_indices() {
            let size = chr.to_digit(10).unwrap() as u8;

            if idx % 2 == 0 {
                files.push(File { pos, size, id });
                id += 1;
            }

            pos += size as u32;
        }

        Self { files }
    }

    fn sort(&mut self) {
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

    fn score(&self) -> u64 {
        let mut sum = 0;
        let mut last = 0;
        let mut idx = 0;

        for x in &self.files {
            idx += x.pos - last;

            sum += (x.id as u64 * x.size as u64 * (x.size as u64 + 2 * idx as u64 - 1)) / 2;
            idx += x.size as u32;

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
