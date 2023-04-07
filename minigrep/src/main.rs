use minigrep::prelude::*;

fn main() {
    /* collect command line argument strings into a vector */
    let args: Vec<String> = env::args().collect();

    /* create a new instance of the Config structure from the command line arguments */
    let config = match Config::new(&args) {
        Ok(cfg) => cfg,
        Err(e) => {
            println!("{}", e);
            exit(1);
        }
    };

    /* run the application with the proivded configuration */
    if let Err(e) = run(config) {
        println!("Application error: {}", e);
        exit(1);
    };
}

