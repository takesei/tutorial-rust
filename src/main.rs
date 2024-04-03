// mod tutorial;
use std::{env, error::Error, process};

use tutorial::Config;

fn main() {
    // tutorial::sample();
    let args = env::args();
    println!("{:?}", args);

    let config = Config::new(args).unwrap_or_else(|why: &str| {
        println!("Arguments parse failed, got:{}", why);
        process::exit(1)
    });

    println!("{} -> {:?}", config.query, config.path);

    let contents = tutorial::run(&config).unwrap_or_else(|why: Box<dyn Error>| {
        println!("runtime error, got:{}", why);
        process::exit(1)
    });
    println!("{}", contents);

    let result = tutorial::search(&config.query, &contents);
    println!("{:?}", result);
}
