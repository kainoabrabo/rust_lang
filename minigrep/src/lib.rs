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

        Ok(Config { s, f })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // get String contents from file
    let contents = fs::read_to_string(config.f).expect("Should be able to read the file.");

    // go through the result list and print each line
    for line in search(&config.s, &contents) {
        println!("'{line}'");
    }

    Ok(())
}

pub fn search<'a>(_s: &str, _contents: &'a str) -> Vec<&'a str> {
    // store result line
    let mut results = Vec::new();

    // .lines() -> iterator
    for line in _contents.lines() {
        
        // match line
        if line.contains(_s) {
            results.push(line);
        }
    }

    results
}

// Test Driven Development

// Test 1
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let s = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(s, contents));
    }
}

