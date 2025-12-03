use common::{Answer, solution};

solution!("Gift Shop", 2);

fn part_a(input: &str) -> Answer {
    count_invalid(input, |id| {
        let digits = id.ilog10().div_ceil(2);
        let mask = u64::pow(10, digits);
        id % mask == id / mask
    })
    .into()
}

fn part_b(input: &str) -> Answer {
    count_invalid(input, |id| {
        let digits = id.ilog10() + 1;
        'outer: for size in 1..=(digits / 2) {
            let mask = u64::pow(10, size);
            let repeated = id % mask;
            if !digits.is_multiple_of(size) {
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
    })
    .into()
}

fn count_invalid(input: &str, is_invalid: fn(u64) -> bool) -> u64 {
    let mut count = 0;

    for range in input.split(',') {
        let (start, end) = range.split_once('-').unwrap();
        let start = start.parse().unwrap();
        let end = end.parse().unwrap();

        for id in start..=end {
            count += is_invalid(id) as u64 * id;
        }
    }

    count
}

#[cfg(test)]
mod test {
    const CASE: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(CASE), 1227775554.into());
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(CASE), 4174379265_u64.into());
    }
}
