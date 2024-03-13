use reqwest;
use serde::{Serialize, Deserialize};
use std::io;
use std::io::Write;
use std::time::Duration;
use tokio::time::sleep;


#[derive(Debug, Serialize, Deserialize)]
struct ExchangeRates {
    conversion_rates: std::collections::HashMap<String, f64>,
}


#[derive(Debug, Serialize, Deserialize)]
struct ErrorResponse {
    result: String,
    #[serde(rename = "error-type")]
    error_type: String,
}


#[derive(Debug, Serialize, Deserialize)]
struct SupportedList {
    supported_codes: Vec<Vec<String>>,
}


async fn display_currencies() -> Result<std::collections::HashMap<String, f64>, reqwest::Error> {
    let list_link = "https://v6.exchangerate-api.com/v6/a3f798577a713b0309d32d40/latest/USD";
        let list_response = reqwest::Client::new().get(list_link).send().await;
        let mut xs:  std::collections::HashMap<String, f64> = std::collections::HashMap::new();
        match list_response {
            Ok(res) => {
                if res.status().is_success() {
                    let list_body: ExchangeRates = res.json().await?;
                    xs = list_body.conversion_rates.clone();
                    for (key, value) in list_body.conversion_rates {
                        println!("Code: {}, Rate: {}", key, value);
                    }
                }
            }
            Err(err) => {
                eprintln!("Failed to parse currencies: {}", err);
            }
        }
    Ok(xs)
}

fn read_value() -> f64 {
    print!("Value to be converted: ");
    io::stdout().flush().expect("Failed to flush");
    let mut input_line = String::new();
    io::stdin()
        .read_line(&mut input_line)
        .expect("Failed to read line");

    let value: f64 = input_line
        .trim()
        .parse()
        .expect("Input not an integer");

    if value<=0.0 {
        println!("Value less or equal to 0, enter valid value");
        return -1.0;
    } else {
        return value;
    } 
}


async fn read_inout_code(av: Option<std::collections::HashMap<String, f64>>) -> Result<(), reqwest::Error>{
    let mut cur_from = String::new();
    print!("Enter base currency: ");
    io::stdout().flush().expect("Failed to flush");
    io::stdin()
    .read_line(&mut cur_from)
    .expect("Failed to read input currency");
    let cur_from = cur_from.trim();

    let mut cur_to = String::new();
    print!("Enter output currency: ");
    io::stdout().flush().expect("Failed to flush");
    io::stdin()
    .read_line(&mut cur_to)
    .expect("Failed to read output currency");
    let cur_to = cur_to.trim();

    if let Some(v) = av {
        let num = read_value();
        if num!=-1.0 {
            if !v.is_empty() {
                if v.contains_key(&cur_from.to_string()) && v.contains_key(&cur_to.to_string()) {
                    let ab = v.get(&cur_from.to_string());
                    match ab {
                        Some(m) => {
                            if *m==1.0 {
                                if let Some(rate) = v.get(&cur_to.to_string()) {
                                    non_api_convert(cur_from.to_string(), cur_to.to_string(), num, *rate);
                                } else {
                                    println!("Error: Currency not found in the HashMap");
                                }
                            } else {
                                api_convert(cur_from.to_string(), cur_to.to_string(), num).await?;
                            }
                        }
                        None => {}
                    }
                } else if v.contains_key(&cur_to.to_string()) {
                    println!("Invalid input currency code: {}", cur_from);
                } else if v.contains_key(&cur_from.to_string()) {
                    println!("Invalid output currency code: {}", cur_to);
                } else {
                    println!("Invalid input & output currency code!");
                }
            } else {
                api_convert(cur_from.to_string(), cur_to.to_string(), num).await?;
            }
        }
    }
    Ok(())
}


fn non_api_convert(from: String, to: String, amount: f64, rate: f64) {
    println!("{:.2} {} exchanged with {} rate is {:.2} {}", amount, &from, rate, amount*rate, &to);
}


async fn api_convert(from: String, to: String, amount: f64) -> Result<(), reqwest::Error> {

    let base = "https://v6.exchangerate-api.com/v6/a3f798577a713b0309d32d40/latest/".to_string();

    let link = base + &from;
    let mut retry_counter = 0;

    loop {
        let response = reqwest::Client::new().get(&link).send().await;

        match response {
            Ok(res) => {
                if res.status().is_success() {
                    let body: ExchangeRates = res.json().await?;
                    if let Some(rate) = body.conversion_rates.get(&to) {
                        println!("{:.2} {} exchanged with {} rate is {:.2} {}", amount, &from, rate, amount*rate, &to);
                        break;
                    } else {
                        println!("Error: Invalid output currency: {}", &to);
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
                    let error_body : ErrorResponse = res.json().await?;
                    match error_body.error_type.as_ref() {
                        "unsupported-code" =>println!("Error: Invalid input currency code: {}", &from),
                        "malformed-request" =>println!("Error: Invalid request structure"),
                        "invalid-key" =>println!("Error: Invalid API key"),
                        "inactive-account" =>println!("Error: Inactive account (email address not confirmed)"),
                        "quota-reached" =>println!("Error: Limit of account's requests exceeded"),
                        _=>println!("Error: Unknown error code"),
                    }
                    return Ok(());
                }
            }
            Err(_err) => {
                eprintln!("Error: Network error");
                return Ok(());
            }
        }
    }
    Ok(())
}



#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {

    let mut curs: std::collections::HashMap<String, f64> = std::collections::HashMap::new();

    loop {
        println!("------------------------------- MENU -------------------------------");
        println!("0 - List all available currencies and exchange rates (for US dollar)");
        println!("1 - Enter base currency (the one you convert from)");
        println!("2 - Exit program");
        print!("Enter option: ");
        io::stdout().flush().expect("Failed to flush menu");
        
        let mut menu = String::new();
        io::stdin()
            .read_line(&mut menu)
            .expect("Failed to read menu option");

        let m_value: i32 = menu
            .trim()
            .parse()
            .expect("Input isn't a valid menu option");


        match m_value {
            0 => {
                curs = display_currencies().await?;
            }
            1 => {
                read_inout_code(Some(curs.clone())).await?;
            }
            2 => {
                println!("Exiting program!");
                break;
            }
            _ => {
                println!("Input isn't a valid menu option");
            }
        }
    }
    Ok(())
}