extern crate reqwest;

mod client;
use client::Client;



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn it_calls() {
        // client::call_func();
        let client = Client::ping();

    }
}
