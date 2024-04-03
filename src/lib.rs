use std::{error::Error, fs::File, io::Read, path::Path};

pub fn run(conf: &Config) -> Result<String, Box<dyn Error>> {
    let mut contents = String::new();

    let mut f = File::open(conf.path)?;

    f.read_to_string(&mut contents)?;

    Ok(contents)
}

pub struct Config<'a> {
    pub fname: &'a String,
    pub path: &'a Path,
}

impl<'a> Config<'a> {
    pub fn new(args: &'a [String]) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("Size of the length must be three");
        }
        let fname = &args[1];
        let path = &args[2];

        Ok(Self {
            fname,
            path: Path::new(path),
        })
    }
}

#[cfg(test)]
mod test {
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
