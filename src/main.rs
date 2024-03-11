use reqwest;
use serde::{Serialize, Deserialize};
use std::io;


#[derive(Debug, Serialize, Deserialize)]
struct ExchangeRates {
    conversion_rates: std::collections::HashMap<String, f64>,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {




    let mut cur_from = String::new();
    println!("Enter base currency: ");
    match io::stdin().read_line(&mut cur_from) {
        Ok(n) => {
            println!("{} bytes read", n);
        }
        Err(error) => println!("error: {error}"),
    }

    println!("Output currency: ");
    let mut cur_to = String::new();
    match io::stdin().read_line(&mut cur_to) {
        Ok(n) => {
            println!("{} bytes read", n);
        }
        Err(error) => println!("error: {error}"),
    }

    let cur_from = cur_from.trim();
    let cur_to = cur_to.trim();

    println!("Value to be converted: ");

    let mut input_line = String::new();
    io::stdin()
        .read_line(&mut input_line)
        .expect("Failed to read line");
    let value: f64 = input_line
        .trim()
        .parse()
        .expect("Input not an integer");

    let mut link = "https://v6.exchangerate-api.com/v6/a3f798577a713b0309d32d40/latest/".to_string();

    link.push_str(&cur_from);

    let body : ExchangeRates = reqwest::Client::new()
        .get(link)
        .send()
        .await?
        .json()
        .await?;

    if let Some(rate) = body.conversion_rates.get(cur_to) {
        println!("{:.2}", value*rate);
    } else {
        println!("{} exchange rate not found in the response.", cur_to);
    }

    Ok(())
}
