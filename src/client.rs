use std::io::Read;
use reqwest::Error;

pub fn call_func() -> Result<(), Error> {
    
    let mut res = reqwest::get("https://api.coingecko.com/api/v3/ping")?;
    let mut body = String::new();
    res.read_to_string(&mut body);

    println!("Status: {}", res.status());
    println!("Headers:\n{:#?}", res.headers());
    println!("Body:\n{}", body);

    Ok(())
}

