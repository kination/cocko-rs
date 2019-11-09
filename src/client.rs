use std::io::Read;
use std::concat;
use reqwest::Error;

pub struct Client {

}

impl Client {
    
    pub fn ping() -> String {
        let api_domain = "https://api.coingecko.com/api/v3";
        let api = format!("{}{}", api_domain, "/ping");

        let resp = reqwest::get(&api);
        let mut res = match resp {
            Ok(res) => res,
            Err(Error) => return String::from("Error!!")
        };

        let mut body = String::new();
        res.read_to_string(&mut body);

        println!("Status: {}", res.status());
        println!("Headers:\n{:#?}", res.headers());

        return String::from("OK")
    }

    fn coin_list() -> String {
        return String::from("OK")
    }
}
