pub mod client;
pub mod network;

mod outermost {
    pub fn middle_function() {}
    fn middle_secret_function() {}
    pub mod inside {
        pub fn inner_function() {}
        fn inner_secret_function() {}
    }
}

fn try_me() {
    outermost::middle_function();
    outermost::inside::inner_function();
}

#[cfg(test)]
mod tests {
    use super::client;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn it_works2() {
        client::connect();
    }
}
