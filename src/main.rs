use reqwest;
use serde::{Serialize, Deserialize};
use std::io;


#[derive(Debug, Serialize, Deserialize)]
struct ExchangeRates {
    conversion_rates: std::collections::HashMap<String, f64>,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {

    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(n) => {
            println!("{} bytes read", n);
        }
        Err(error) => println!("error: {error}"),
    }

    let link = "https://v6.exchangerate-api.com/v6/a3f798577a713b0309d32d40/latest/";


    let body : ExchangeRates = reqwest::Client::new()
    .get(link)
    .send()
    .await?
    .json()
    .await?;

    //println!("{:#?}", body);

    if let Some(usd_exchange_rate) = body.conversion_rates.get("USD") {
        println!("Exchange rate for USD: {}", usd_exchange_rate);
    } else {
        println!("USD exchange rate not found in the response.");
    }

    Ok(())
}
