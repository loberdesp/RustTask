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

async fn display_currencies(map: &mut std::collections::HashMap<String, std::collections::HashMap<String, f64>>) -> Result<(), reqwest::Error> {

    if let Some(usd_map) = map.get("USD") {
        for (key, value) in usd_map {
            println!("Code: {}, Rate: {}", key, value);
        }
    } else {
        let list_link = "https://v6.exchangerate-api.com/v6/a3f798577a713b0309d32d40/latest/USD";
        let list_response = reqwest::Client::new().get(list_link).send().await;
        match list_response {
            Ok(res) => {
                if res.status().is_success() {
                    let list_body: ExchangeRates = res.json().await?;
                    map.insert("USD".to_string(), list_body.conversion_rates.clone());
                    for (key, value) in list_body.conversion_rates {
                        println!("Code: {}, Rate: {}", key, value);
                    }
                }
            }
            Err(err) => {
                eprintln!("Failed to parse currencies: {}", err);
            }
        }
    }
    Ok(())
}

fn read_value() -> f64 {
    print!("Value to be converted (e.g., 14.26): ");
    io::stdout().flush().expect("Failed to flush");
    let mut input_line = String::new();
    io::stdin()
        .read_line(&mut input_line)
        .expect("Failed to read line");
    let value: f64 = input_line
        .trim()
        .parse()
        .expect("Input not a valid floating-point number");
    if value <= 0.0 {
        println!("Value less than or equal to 0, enter a valid value");
        return -1.0;
    } else {
        return value;
    }
}

fn is_uppercase(input: &str) -> bool {
    input.chars().all(char::is_uppercase)
}

async fn read_input_code(available_currencies: &mut std::collections::HashMap<String, std::collections::HashMap<String, f64>>) -> Result<(), reqwest::Error> {
    let mut cur_from = String::new();
    let mut cur_to = String::new();

    loop {
        print!("Enter base currency (e.g., USD): ");
        io::stdout().flush().expect("Failed to flush base currency");
        io::stdin().read_line(&mut cur_from).expect("Failed to read input currency");
        cur_from = cur_from.trim().to_string();

        if is_uppercase(&cur_from) {
            break;
        } else {
            println!("Invalid input. It should only contain uppercase characters!");
            cur_from.clear();
        }
    }

    loop {
        print!("Enter output currency (e.g., GBP): ");
        io::stdout().flush().expect("Failed to flush output currency");
        io::stdin().read_line(&mut cur_to).expect("Failed to read output currency");
        cur_to = cur_to.trim().to_string();

        if is_uppercase(&cur_to) {
            break;
        } else {
            println!("Invalid input. It should only contain uppercase characters!");
            cur_to.clear();
        }
    }

    let num = read_value();
    if num != -1.0 {
        if let Some(rate_from) = available_currencies.get(&cur_from) {
            if let Some(rate_to) = rate_from.get(&cur_to) {
                non_api_convert(cur_from.to_string(), cur_to.to_string(), num, *rate_to);
            } else {
                api_convert(cur_from.to_string(), cur_to.to_string(), num, available_currencies).await?;
            }
        } else {
            api_convert(cur_from.to_string(), cur_to.to_string(), num, available_currencies).await?;
        }
    }
    Ok(())
}

fn non_api_convert(from: String, to: String, amount: f64, rate: f64) {
    println!("{:.2} {} exchanged with {} rate is {:.2} {}", amount, &from, rate, amount * rate, &to);
}

async fn api_convert(from: String, to: String, amount: f64, map: &mut std::collections::HashMap<String, std::collections::HashMap<String, f64>>) -> Result<(), reqwest::Error> {
    let base = "https://v6.exchangerate-api.com/v6/a3f798577a713b0309d32d40/latest/".to_string();
    let link = base + &from;
    let mut retry_counter = 0;

    loop {
        let response = reqwest::Client::new().get(&link).send().await;

        match response {
            Ok(res) => {
                if res.status().is_success() {
                    let body: ExchangeRates = res.json().await?;
                    let _ = map.insert(from.clone(), body.conversion_rates.clone());
                    if let Some(rate) = body.conversion_rates.get(&to) {
                        println!("{:.2} {} exchanged with {} rate is {:.2} {}", amount, &from, rate, amount * rate, &to);
                        break;
                    } else {
                        println!("Error: Invalid output currency: {}", &to);
                        break;
                    }
                } else if res.status() == reqwest::StatusCode::TOO_MANY_REQUESTS {
                    eprintln!("Rate limit exceeded. Retrying after delay...");
                    sleep(Duration::from_secs(5)).await;
                    retry_counter += 1;
                    if retry_counter > 3 {
                        eprintln!("Maximum retries reached. Exiting.");
                        return Ok(());
                    }
                } else {
                    let error_body: ErrorResponse = res.json().await?;
                    match error_body.error_type.as_ref() {
                        "unsupported-code" => println!("Error: Invalid input currency code: {}", &from),
                        "malformed-request" => println!("Error: Invalid request structure"),
                        "invalid-key" => println!("Error: Invalid API key"),
                        "inactive-account" => println!("Error: Inactive account (email address not confirmed)"),
                        "quota-reached" => println!("Error: Limit of account's requests exceeded"),
                        _ => println!("Error: Unknown error code"),
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
    let mut curs: std::collections::HashMap<String, std::collections::HashMap<String, f64>> = std::collections::HashMap::new();

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
                display_currencies(&mut curs).await?;
            }
            1 => {
                read_input_code(&mut curs).await?;
            }
            2 => {
                println!("Exiting program!");
                break;
            }
            _ => {
                println!("Input isn't a valid menu option");
            }
        }

        //debug purpose code below, display all cached exchanged rates to optimize and reduce api calls

        // for(key, _value) in &curs {
        //     println!("{:?}", key)
        // }
    }
    Ok(())
}
