use std::{borrow::Cow, fmt::Display};

use anyhow::{bail, Context, Result};

use self::tokenize::Tokenizer;

#[derive(Debug)]
pub struct Formatter {
    components: Box<[Component]>,
}

#[derive(Debug)]
pub enum Component {
    Literal(String),
    Format {
        name: String,
        processors: Box<[Processor]>,
    },
}

#[derive(Debug)]
pub enum Processor {
    Pad { width: usize },
    Uppercase,
}

pub trait Arguments {
    fn get(&self, name: &str) -> Option<Cow<'_, str>>;
}

impl Formatter {
    pub fn new(format: &str) -> Result<Self> {
        let components = Tokenizer::new(format).tokenize()?.into();
        Ok(Self { components })
    }

    pub fn format<T: Arguments>(&self, args: T) -> Result<String> {
        let mut output = String::new();
        for component in self.components.iter() {
            match component {
                Component::Literal(literal) => output.push_str(literal),
                Component::Format { name, processors } => {
                    let value = args
                        .get(name)
                        .with_context(|| format!("No argument named {name} found"))?;

                    let mut value = value.to_string();
                    for processor in processors.iter() {
                        value = processor.process(&value)?;
                    }

                    output.push_str(&value);
                }
            }
        }

        Ok(output)
    }
}

impl Processor {
    fn parse(name: &str, args: &str) -> Result<Self> {
        Ok(match name.to_lowercase().as_str() {
            "pad" => {
                let width = args
                    .parse()
                    .with_context(|| format!("Invalid width of `{args}`"))?;
                Self::Pad { width }
            }
            "uppercase" => Self::Uppercase,
            _ => bail!("Unknown processor: {}", name),
        })
    }

    fn process(&self, input: &str) -> Result<String> {
        Ok(match self {
            Self::Pad { width } => {
                let mut output = input.to_string();
                while output.len() < *width {
                    output.insert(0, '0');
                }
                output
            }
            Self::Uppercase => input.to_uppercase(),
        })
    }
}

impl<T: Display> Arguments for &[(&str, T)] {
    fn get(&self, name: &str) -> Option<Cow<'_, str>> {
        self.iter()
            .find(|(key, _)| key == &name)
            .map(|(_, value)| Cow::Owned(value.to_string()))
    }
}

mod tokenize {
    use anyhow::{Context, Result};

    use super::{Component, Processor};

    pub struct Tokenizer {
        input: Box<[char]>,
        index: usize,
        output: Vec<Component>,
    }

    impl Tokenizer {
        pub fn new(input: &str) -> Self {
            Self {
                input: input.chars().collect(),
                index: 0,
                output: Vec::new(),
            }
        }

        pub fn tokenize(mut self) -> Result<Vec<Component>> {
            while self.index < self.input.len() {
                let c = self.input[self.index];
                match c {
                    // TODO: This can go out of bounds
                    '{' if self.input[self.index + 1] == '{' => self.tokenize_format()?,
                    _ => self.tokenize_literal(),
                }
            }

            Ok(self.output)
        }

        fn tokenize_literal(&mut self) {
            let mut literal = String::new();
            let mut past_start = false;
            while self.index < self.input.len() {
                let c = self.input[self.index];
                match c {
                    '{' if past_start => break,
                    _ => {
                        literal.push(c);
                        past_start = true;
                    }
                }
                self.index += 1;
            }

            self.output.push(Component::Literal(literal));
        }

        fn tokenize_format(&mut self) -> Result<()> {
            self.index += 2;
            let mut name = String::new();
            let mut processors = Vec::new();
            while self.index < self.input.len() {
                let c = self.input[self.index];
                match c {
                    '}' => {
                        self.index += 1;
                        break;
                    }
                    ':' => {
                        processors.push(self.parse_processor().context("Parsing processor")?);
                        continue;
                    }
                    _ => name.push(c),
                }
                self.index += 1;
            }

            self.index += 1;
            self.output.push(Component::Format {
                name,
                processors: processors.into(),
            });
            Ok(())
        }

        fn parse_processor(&mut self) -> Result<Processor> {
            self.index += 1;
            let mut name = String::new();
            while self.index < self.input.len() {
                let c = self.input[self.index];
                match c {
                    '}' => {
                        break;
                    }
                    ':' => {
                        break;
                    }
                    _ => name.push(c),
                }
                self.index += 1;
            }

            let (name, args) = name.split_once('(').unwrap_or((&name, ""));
            let args = &args[..args.len().saturating_sub(1)];
            Processor::parse(name.to_lowercase().as_str(), args)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let formatter = Formatter::new("Hello, {{title:pad(2)}}.{{name}}!").unwrap();
        let output = formatter
            .format::<&[(&str, &str)]>(&[("title", "Mr"), ("name", "John")])
            .unwrap();
        assert_eq!(output, "Hello, Mr.John!");
    }
}
