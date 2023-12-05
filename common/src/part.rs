use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
pub enum Part {
    A,
    B,
}

impl FromStr for Part {
    type Err = String;

    fn from_str(s: &str) -> Result<Part, Self::Err> {
        match s {
            "a" => Ok(Part::A),
            "b" => Ok(Part::B),
            _ => Err("part must be `a` or `b`".to_owned()),
        }
    }
}

impl ToString for Part {
    fn to_string(&self) -> String {
        match self {
            Part::A => "a",
            Part::B => "b",
        }
        .to_owned()
    }
}
