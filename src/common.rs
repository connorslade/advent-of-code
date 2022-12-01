const TIME_UNITS: &[&str] = &["ns", "μs", "ms", "s"];

pub fn time_unit(time: u128) -> String {
    let mut time = time;
    for i in TIME_UNITS {
        if time < 1000 {
            return format!("{}{}", time, i);
        }
        time /= 1000;
    }

    format!("{}{}", time, TIME_UNITS.last().unwrap())
}
