use std::error::Error;
use std::fs;

const WORDLIST_FILENAME: &str = "bip39.txt";

pub struct Config<'a> {
    wordlist: String,
    filename: &'a str
}

impl<'a> Config<'a> {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }

        let wordlist = fs::read_to_string(WORDLIST_FILENAME)
            .expect(&format!("Could not read {}", WORDLIST_FILENAME));

        Ok(Config {
            wordlist,
            filename: &args[1]
        })
    }
}

pub fn run(config: Config) -> Result<Vec<String>, Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)
        .expect(&format!("Could not read {}", &config.filename));

    let mut matches = Vec::new();

    for word in config.wordlist.split("\n") {
        if search(word, &contents.split(" ").collect()) == true {
            println!("{}", word);
            matches.push(String::from(word));
        }
    }

    Ok(matches)
}

pub fn search(word: &str, contents: &String) -> bool {
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
        let contents = String::from("apple\nbanana\nzoo");

        assert_eq!(search("apple", &contents), true);
    }

    #[test]
    fn search_finds_word_false() {
        let contents = String::from("apple\nbanana\nzoo");

        assert_eq!(search("pear", &contents), false);
    }

    #[test]
    fn search_respects_casing() {
        assert_eq!(search("pear", &String::from("Pear")), false);
    }
}
