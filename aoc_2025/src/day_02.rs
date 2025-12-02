use common::{Answer, solution};

solution!("Gift Shop", 2);

fn part_a(input: &str) -> Answer {
    let mut count = 0;

    for range in input.trim().split(',') {
        let (start, end) = range.split_once('-').unwrap();
        let start = start.parse::<u64>().unwrap();
        let end = end.parse::<u64>().unwrap();

        for id in start..=end {
            count += is_invalid(id) as u64 * id;
        }
    }

    count.into()
}

fn part_b(input: &str) -> Answer {
    let mut count = 0;

    for range in input.trim().split(',') {
        let (start, end) = range.split_once('-').unwrap();
        let start = start.parse::<u64>().unwrap();
        let end = end.parse::<u64>().unwrap();

        for id in start..=end {
            count += is_invalid2(id) as u64 * id;
        }
    }

    count.into()
}

fn is_invalid(id: u64) -> bool {
    let digits = (id.ilog10() + 1) / 2;
    let mask = 10_u64.pow(digits);
    id % mask == id / mask
}

fn is_invalid2(id: u64) -> bool {
    let digits = id.ilog10() + 1;

    'outer: for size in 1..=(digits / 2) {
        let mask = 10_u64.pow(size);
        let repeated = id % mask;
        if (digits / size) * size != digits || repeated == 0 || repeated.ilog10() + 1 != size {
            continue;
        }

        let mut id = id;
        for _ in 0..(digits / size) {
            if id % mask != repeated {
                continue 'outer;
            }

            id /= mask;
        }

        return true;
    }

    false
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    const CASE: &str = indoc! {"
        11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124
    "};

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(CASE), 1227775554.into());
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(CASE), 4174379265_u64.into());
    }
}
