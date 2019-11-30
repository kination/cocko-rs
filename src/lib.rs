extern crate reqwest;
#[macro_use]
extern crate json;

mod client;
mod simple;
mod coins;

use client::CockoClient;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn call_ping() {
        // client::call_func();
        /*
        let client = CockoClient::ping();

        match client {
            Ok(result) => println!("Ping success: {:?}", result["gecko_says"]),
            Err(msg) => println!("Error!!")
        }
        */
    }

    #[test]
    fn call_simple_price() {
        let client = CockoClient::simple_price();
    }


}
