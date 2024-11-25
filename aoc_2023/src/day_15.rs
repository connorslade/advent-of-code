use common::{solution, Answer};
use itertools::Itertools;

solution!("Lens Library", 15);

fn part_a(input: &str) -> Answer {
    input
        .trim()
        .split(',')
        .map(|x| hash(x) as u32)
        .sum::<u32>()
        .into()
}

fn part_b(input: &str) -> Answer {
    let input = parse(input);
    let mut boxes = vec![Vec::new(); 256];

    for (label, focal_len) in input {
        let key = hash(label) as usize;
        if let Some(focal_len) = focal_len {
            if let Some((_, e)) = boxes[key]
                .iter_mut()
                .find(|x: &&mut (&str, u32)| x.0 == label)
            {
                *e = focal_len;
            } else {
                boxes[key].push((label, focal_len));
            }
        } else {
            boxes[key].retain(|x| x.0 != label);
        }
    }

    let mut acc = 0;
    for (i, e) in boxes.iter().enumerate() {
        for (j, f) in e.iter().enumerate() {
            acc += (i + 1) * (j + 1) * f.1 as usize;
        }
    }

    acc.into()
}

fn parse(input: &str) -> Vec<(&str, Option<u32>)> {
    let mut out = Vec::new();

    for i in input.trim().split(',') {
        let (label, focal_len) = i.split(['=', '-'].as_ref()).collect_tuple().unwrap();
        out.push((label, focal_len.parse::<u32>().ok()));
    }

    out
}

fn hash(input: &str) -> u8 {
    let mut out = 0u8;
    for c in input.chars() {
        out = out.wrapping_add(c as u8).wrapping_mul(17);
    }

    out
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    const CASE: &str = indoc! {"
        rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7
    "};

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(CASE), 1320.into());
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(CASE), 145.into());
    }
}
