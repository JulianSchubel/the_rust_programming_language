pub mod prelude {
    pub use std::{env, process::exit};
    use std::{fs, error::Error};
    pub struct Config<'a> {
        /* the query to run against a block of text */
        pub query: & 'a String,
        /* filename of the file containing text to be queried */
        pub filename: & 'a String,
    }

    impl<'a> Config<'a> {
            pub fn new(args: & 'a Vec<String>) -> Result<Config, & 'static str> {
                if args.len() < 3 {
                    return Err("Usage: minigrep <pattern> <filename>");
                }
                Ok(Self {
                    query: &args[1],
                    filename: &args[2],
                })
            }
    }

    pub fn search<'a>(query: &str, contents: & 'a str) -> Vec<& 'a str> {
        let mut v: Vec<&str> = Vec::new();
        for line in contents.lines() {
            if line.find(query) != None {
                v.push(line);
            }
        }
        v
    }

    pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
        let contents: String = fs::read_to_string(config.filename)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::prelude::*;

    /* Test that a single line is returned */
    #[test]
    fn test_one_result() {
        let query = "frog";
        let contents = "I'm nobody! Who are you?
Are you nobody, too?
Then there's a pair of us — don't tell!
They'd banish us, you know.
How dreary to be somebody!
How public, like a frog
To tell your name the livelong day
To an admiring bog!";
       assert_eq!(vec!["How public, like a frog"], search(query, contents));
    }
}
