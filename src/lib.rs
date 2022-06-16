use std::error::Error;
use std::fs;

pub struct Config<'a> {
    query: &'a str,
    filename: &'a str
}

impl<'a> Config<'a> {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        Ok(Config {
            query: &args[1],
            filename: &args[2]
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)
        .expect(&format!("Could not read {}", &config.filename));

    println!("{}", contents);

    Ok(())
}

// search a word from a vector
pub fn search(word: &str, contents: Vec<&str>) -> bool {
    if contents.contains(&word) {
        return true
    }

    false
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_finds_word_true() {
        let contents = vec!["apple", "banana", "zoo"];

        assert_eq!(search("zoo", contents), true);
    }

    #[test]
    fn search_finds_word_false() {
        let contents = vec!["apple", "banana", "zoo"];

        assert_eq!(search("pear", contents), false);
    }

    #[test]
    fn search_respects_casing() {
        assert_eq!(search("pear", vec!["Pear"]), false);
    }
}
