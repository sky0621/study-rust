#[cfg(test)]
mod tests {
    use crate::adder::{greeting, Guess};

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn exploration() {
        assert_ne!(2 + 1, 4);
    }

    #[test]
    #[should_panic]
    fn another() {
        panic!("Make this test fail");
    }

    #[test]
    fn greeting_contains_name() {
        let res = greeting("Taro");
        assert!(
            res.contains("Taro"),
            "Greeting did not contain name, value was '{}'",
            res
        );
    }

    #[test]
    #[should_panic(expected = "Guess value must be between 1 and 100, got 101")]
    fn greater_than_100() {
        Guess::new(101);
    }
}

pub fn greeting(name: &str) -> String {
    String::from(name)
}

pub struct Guess {
    value: u32,
}

impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}", value);
        }
        Guess { value }
    }
}
