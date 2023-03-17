pub mod prelude {

    pub use std::fs;

    pub struct Config<'a> {
        pub query: & 'a String,
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

    pub fn minigrep(query: &str, filename: &str) -> String {
        let contents: String =  match fs::read_to_string(filename) {
            Ok(s) => s,
            Err(_) => panic!("Failed to read file \"{}\"", filename),
        };
        contents
    }
}
