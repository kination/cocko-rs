extern crate reqwest;


mod client;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn it_calls() {
        client::call_func();
    }
}
