use reqwest;
use serde::{Serialize, Deserialize};
use std::io;
use std::time::Duration;
use tokio::time::sleep;

#[derive(Debug, Serialize, Deserialize)]
struct ExchangeRates {
    conversion_rates: std::collections::HashMap<String, f64>,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {

    let mut cur_from = String::new();
    println!("Enter base currency: ");
    io::stdin()
    .read_line(&mut cur_from)
    .expect("Failed to read input currency");

    println!("Enter output currency: ");
    let mut cur_to = String::new();
    io::stdin()
    .read_line(&mut cur_to)
    .expect("Falied to read output currency");

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

    let mut retry_counter = 0;

    loop {
        let response = reqwest::Client::new().get(&link).send().await;

        match response {
            Ok(res) => {
                if res.status().is_success() {
                    let body: ExchangeRates = res.json().await?;
                    if let Some(rate) = body.conversion_rates.get(cur_to) {
                        println!("{} {} exchanged with {} rate is {:.2} {}", value, cur_from, rate, value*rate, cur_to);
                        break;
                    } else {
                        println!("{} exchange rate not found in the response.", cur_to);
                        break;
                    }
                } else if res.status() == reqwest::StatusCode::TOO_MANY_REQUESTS {
                    eprintln!("Rate limit exceeded. Retrying after delay...");

                    // Wait for a moment before retrying (you may adjust the duration)
                    sleep(Duration::from_secs(5)).await;

                    // Increment retry counter and check if exceeded maximum retries
                    retry_counter += 1;
                    if retry_counter > 3 {
                        eprintln!("Maximum retries reached. Exiting.");
                        return Ok(());
                    }
                } else {
                    eprintln!("Unexpected response: {:?}", res);
                    return Ok(());
                }
            }
            Err(err) => {
                eprintln!("Network error: {}", err);
                return Ok(());
            }
        }
    }

    Ok(())
}





