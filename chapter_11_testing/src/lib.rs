#![warn(clippy::all, clippy::pedantic)]

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn greeting(name: &str) -> String {
    format!("{name}")
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

/* Private function for testing */
fn internal_adder(a: i32, b:i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    /* test that errors are handled and returned: no panics! */
    fn exploration_result() -> Result<(), String> {
        if add(2,2) != 4 {
            Err(String::from("2 + 2 must equal four"))
        } else {
            Ok(())
        }
    }

    #[test]
    fn another() {
        panic!("Make this test fail");
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5, 
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5, 
            height: 1,
        };
        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add(2, 2));
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(result.contains("Carol"), "Greeting did not contain name, value was `{}`", result);
    }

    struct Guess {
        value: i32,
    }

    impl Guess {
        pub fn new(value: i32) -> Self {
            if value < 0 || value > 100 {
                panic!("Value for guess must be in range [0,100]. Got {value}");
            }
            Guess {
                value
            }
        }
    }

    /* Check that a function handles errors correctly */
    #[test]
    #[should_panic]
    fn guess_in_range() {
        Guess::new(200);
    }

    /* More precise should_panic with expected argument */
    #[test]
    #[should_panic(expected = "in range [0,100]")]
    fn guess_in_range_precise() {
        Guess::new(200);
    }

    #[test]
    #[ignore]
    fn expensive_test() {
        /* code that takes an hour to run */
    }

    /* Can test private functions */
    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2,2));
    }
}



