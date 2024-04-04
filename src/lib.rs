use std::{error::Error, fs::File, io::Read};

pub fn run(conf: &Config) -> Result<String, Box<dyn Error>> {
    let mut contents = String::new();

    let mut f = File::open(&conf.path)?;

    f.read_to_string(&mut contents)?;

    Ok(contents)
}

pub struct Config {
    pub query: String,
    pub path: String,
}

impl Config {
    pub fn new(mut args: impl Iterator<Item = String>) -> Result<Self, &'static str> {
        args.next();

        let query = match args.next() {
            Some(val) => val,
            None => return Err("Size of the length must be three"),
        };

        let path = match args.next() {
            Some(val) => val,
            None => return Err("Size of the length must be three"),
        };

        Ok(Self { query, path })
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

#[cfg(test)]
mod test {
    use crate::search;

    #[test]
    fn one_result() {
        let query: &str = "duct";
        let contents = "Rust:
Safe and fast and productive";

        assert_eq!(
            vec!["Safe and fast and productive"],
            search(query, contents)
        )
    }

    #[test]
    fn some_result() {
        let query: &str = "duct";
        let contents = "Rust:
Safe and fast and productive
fast and productive and safe";

        assert_eq!(
            vec![
                "Safe and fast and productive",
                "fast and productive and safe"
            ],
            search(query, contents)
        )
    }
}
