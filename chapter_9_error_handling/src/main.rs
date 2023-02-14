use std::fs::File;
use std::io::{self, Read};
use std::net::IpAddr;

fn simple_panic() -> () {
    panic!("Crash and burn");
}

fn buffer_overread() -> u32 {
    let v = vec![1,2,3];
    v[99]
}

fn read_username_from_file(s: &str) -> Result<String, io::Error> {
    let username_file_result = File::open(s);
    let mut username_file = match username_file_result {
        Ok(file) => file,
        /* propogate the error to calling function */
        Err(e) => return Err(e),
    };

    let mut username: String = String::new();
    match username_file.read_to_string(& mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

/* read_username_from_file using the error propogation operator ? */
/* ? placed after a Result value evaluates to the value in the Ok() variant or returns the Err(e) variant to the calling function */
fn read_username2(s: &str) -> Result<String, io::Error> {
    let mut username_file_result = File::open(s)?;
    let mut username = String::new();
    username_file_result.read_to_string(& mut username)?;
    Ok(username)
}

/* function above but chaining method calls */
fn read_username3(s: &str) -> Result<String, io::Error> {
    let mut username = String::new();
    /* Method calls chained here */
    let mut username_file_result = File::open(s)?.read_to_string(& mut username)?;
    Ok(username)
}

/* Abbreviated form of the above as would be used in actual code using the std::fs::read_to_string */
fn read_file(s: &str) -> Result<String, io::Error> {
    std::fs::read_to_string(s)
}

fn main() {
    /* panic! examples */
    //simple_panic();
    //buffer_overread();

    /* Result examples */
    /* File::open returns a Result enum */
    /* std::fs::File is a file handle. The T generic type will be substituted with this value on success */
    /* std::io::Error is the type subsituted for E on failure */
    let file_result = File::open("./hello.txt");
    
    /* Handle the Result via match and shadowing (we will be mutating the data type of file_result var) */
    let file_result_a = match file_result {
        Ok(file) => file,
        Err(error) => panic!("Unable to open the file: {:?}", error),
    };

    /* Matching on different errors */
    /* Above will panic! on any error during File::open(), take failure specific action with an
     * inner match as below */
    let file_result_b = match File::open("hello.txt") {
        Ok(file) => file, 
        Err(error) => match error.kind() {
            io::ErrorKind::NotFound => match File::create("./hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Unable to open the file {:?}", other_error);
            },
        },
    };

    /* The above using a closure instead and the unwrap_or_else method */
    let file_result_c = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == io::ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Unable to create the file: {:?}", error);
           })
        } else {
            panic!("Unable to open the file: {:?}", error);
        }
    });

    /* A case where we have more information than the compiler */
    /* We are using a hardcoded IP address and we know that 127.0.0.1 is a valid IP address (it's
     * the loopback address) so it is acceptable to use expect() here as we know this will not fail */
    let home: IpAddr = "127.0.0.1".parse().expect("Hardcoded IP address should be valid");
}

/* Main returning Result<(), E> with the Box<dyn Error> trait object as error type (meaning any
 * kind of error)
fn main() -> Result<(), Box<dyn Error>> {
    let greeting_file = File::open("greeting.txt")?;
    Ok(());
}

*/
