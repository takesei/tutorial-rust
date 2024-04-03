// mod tutorial;
use std::{env, error::Error, process};

use tutorial::Config;

fn main() {
    // tutorial::sample();
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let config = Config::new(&args).unwrap_or_else(|why: &str| {
        println!("Arguments parse failed, got:{}", why);
        process::exit(1)
    });

    println!("{}/{:?}", config.fname, config.path);

    let contents = tutorial::run(&config).unwrap_or_else(|why: Box<dyn Error>| {
        println!("runtime error, got:{}", why);
        process::exit(1)
    });
    println!("{}", contents);
}
