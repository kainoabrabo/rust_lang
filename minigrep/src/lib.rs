use std::error::Error;
use std::fs;

pub struct Config {
    pub s: String,
    pub f: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not Enough Arguments");
        }
        let s = args[1].clone();
        let f = args[2].clone();

        Ok(Config {s, f})
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.f).expect("Should be able to read the file.");
    
    println!("Text:\n{contents}");

    Ok(())
}
