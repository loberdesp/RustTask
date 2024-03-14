use reqwest; // Importing the reqwest crate for making HTTP requests.
use serde::{Serialize, Deserialize}; // Importing serde for serialization and deserialization.
use std::io; // Importing std::io module for input and output operations.
use std::io::Write; // Importing Write trait for flushing output.
use std::time::Duration; // Importing Duration for specifying time durations.
use tokio::time::sleep; // Importing sleep function from tokio for asynchronous sleep.

#[derive(Debug, Serialize, Deserialize)] // Implementing Serialize and Deserialize traits for ExchangeRates struct.
struct ExchangeRates {
    conversion_rates: std::collections::HashMap<String, f64>, // Struct representing exchange rates.
}

#[derive(Debug, Serialize, Deserialize)] // Implementing Serialize and Deserialize traits for ErrorResponse struct.
struct ErrorResponse {
    result: String, // Result message.
    #[serde(rename = "error-type")]
    error_type: String, // Error type message.
}

#[derive(Debug, Serialize, Deserialize)] // Implementing Serialize and Deserialize traits for SupportedList struct.
struct SupportedList {
    supported_codes: Vec<Vec<String>>, // Struct representing supported currency codes.
}

async fn display_currencies(map: &mut std::collections::HashMap<String, std::collections::HashMap<String, f64>>) -> Result<(), reqwest::Error> {
    // Function to display available currencies and their exchange rates.
    if let Some(usd_map) = map.get("USD") { // Checking if USD exchange rates are already available.
        for (key, value) in usd_map {
            println!("Code: {}, Rate: {}", key, value); // Printing currency code and exchange rate.
        }
    } else {
        let list_link = "https://v6.exchangerate-api.com/v6/a3f798577a713b0309d32d40/latest/USD"; // API endpoint for getting currency exchange rates.
        let list_response = reqwest::Client::new().get(list_link).send().await; // Sending HTTP GET request to get exchange rates.
        match list_response {
            Ok(res) => {
                if res.status().is_success() { // Checking if the request was successful.
                    let list_body: ExchangeRates = res.json().await?; // Parsing JSON response into ExchangeRates struct.
                    map.insert("USD".to_string(), list_body.conversion_rates.clone()); // Inserting USD exchange rates into the map.
                    for (key, value) in list_body.conversion_rates {
                        println!("Code: {}, Rate: {}", key, value); // Printing currency code and exchange rate.
                    }
                }
            }
            Err(err) => {
                eprintln!("Failed to parse currencies: {}", err); // Printing error if failed to parse currencies.
            }
        }
    }
    Ok(())
}

fn read_value() -> f64 {
    // Function to read a floating-point value from the user.
    print!("Value to be converted (e.g., 14.26): ");
    io::stdout().flush().expect("Failed to flush"); // Flushing stdout.
    let mut input_line = String::new(); // Creating a new string to store user input.
    io::stdin()
        .read_line(&mut input_line)
        .expect("Failed to read line"); // Reading user input.
    let value: f64 = input_line
        .trim()
        .parse()
        .expect("Input not a valid floating-point number"); // Parsing user input to a floating-point number.
    if value <= 0.0 {
        println!("Value less than or equal to 0, enter a valid value"); // Printing error message for invalid input.
        return -1.0;
    } else {
        return value;
    }
}

fn is_uppercase(input: &str) -> bool {
    // Function to check if a string is uppercase.
    input.chars().all(char::is_uppercase) // Checking if all characters in the string are uppercase.
}

async fn read_input_code(available_currencies: &mut std::collections::HashMap<String, std::collections::HashMap<String, f64>>) -> Result<(), reqwest::Error> {
    // Function to read input currency codes and perform conversions.
    let mut cur_from = String::new(); // Initializing a string to store input currency code.
    let mut cur_to = String::new(); // Initializing a string to store output currency code.

    loop {
        print!("Enter base currency (e.g., USD): ");
        io::stdout().flush().expect("Failed to flush base currency"); // Flushing stdout.
        io::stdin().read_line(&mut cur_from).expect("Failed to read input currency"); // Reading input currency code.
        cur_from = cur_from.trim().to_string(); // Trimming whitespace and converting to string.

        if is_uppercase(&cur_from) { // Checking if input currency code is uppercase.
            break; // Exiting loop if input is valid.
        } else {
            println!("Invalid input. It should only contain uppercase characters!"); // Printing error message for invalid input.
            cur_from.clear(); // Clearing the input string.
        }
    }

    loop {
        print!("Enter output currency (e.g., GBP): ");
        io::stdout().flush().expect("Failed to flush output currency"); // Flushing stdout.
        io::stdin().read_line(&mut cur_to).expect("Failed to read output currency"); // Reading output currency code.
        cur_to = cur_to.trim().to_string(); // Trimming whitespace and converting to string.

        if is_uppercase(&cur_to) { // Checking if output currency code is uppercase.
            break; // Exiting loop if input is valid.
        } else {
            println!("Invalid input. It should only contain uppercase characters!"); // Printing error message for invalid input.
            cur_to.clear(); // Clearing the input string.
        }
    }

    let num = read_value(); // Calling read_value function to read conversion value.
    if num != -1.0 {
        if let Some(rate_from) = available_currencies.get(&cur_from) { // Checking if exchange rates for input currency are available.
            if let Some(rate_to) = rate_from.get(&cur_to) { // Checking if exchange rate for output currency is available.
                non_api_convert(cur_from.to_string(), cur_to.to_string(), num, *rate_to); // Performing conversion without API.
            } else {
                api_convert(cur_from.to_string(), cur_to.to_string(), num, available_currencies).await?; // Performing conversion with API.
            }
        } else {
            api_convert(cur_from.to_string(), cur_to.to_string(), num, available_currencies).await?; // Performing conversion with API.
        }
    }
    Ok(())
}

fn non_api_convert(from: String, to: String, amount: f64, rate: f64) {
    // Function to perform conversion without using API.
    println!("{:.2} {} exchanged with {} rate is {:.2} {}", amount, &from, rate, amount * rate, &to); // Printing conversion result.
}

async fn api_convert(from: String, to: String, amount: f64, map: &mut std::collections::HashMap<String, std::collections::HashMap<String, f64>>) -> Result<(), reqwest::Error> {
    // Function to perform conversion using API.
    let base = "https://v6.exchangerate-api.com/v6/a3f798577a713b0309d32d40/latest/".to_string(); // API base URL.
    let link = base + &from; // Creating API endpoint URL.
    let mut retry_counter = 0; // Initializing retry counter.

    loop {
        let response = reqwest::Client::new().get(&link).send().await; // Sending HTTP GET request to API.

        match response {
            Ok(res) => {
                if res.status().is_success() { // Checking if the request was successful.
                    let body: ExchangeRates = res.json().await?; // Parsing JSON response into ExchangeRates struct.
                    let _ = map.insert(from.clone(), body.conversion_rates.clone()); // Inserting exchange rates into the map.
                    if let Some(rate) = body.conversion_rates.get(&to) { // Checking if exchange rate for output currency is available.
                        println!("{:.2} {} exchanged with {} rate is {:.2} {}", amount, &from, rate, amount * rate, &to); // Printing conversion result.
                        break; // Exiting loop.
                    } else {
                        println!("Error: Invalid output currency: {}", &to); // Printing error message for invalid output currency.
                        break; // Exiting loop.
                    }
                } else if res.status() == reqwest::StatusCode::TOO_MANY_REQUESTS { // Checking if rate limit exceeded.
                    eprintln!("Rate limit exceeded. Retrying after delay...");
                    sleep(Duration::from_secs(5)).await; // Sleeping for 5 seconds.
                    retry_counter += 1; // Incrementing retry counter.
                    if retry_counter > 3 { // Checking if maximum retries reached.
                        eprintln!("Maximum retries reached. Exiting.");
                        return Ok(());
                    }
                } else {
                    let error_body: ErrorResponse = res.json().await?; // Parsing JSON error response into ErrorResponse struct.
                    match error_body.error_type.as_ref() { // Matching error type.
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
                eprintln!("Error: Network error"); // Printing network error message.
                return Ok(());
            }
        }
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    // Main function for currency conversion program.
    let mut curs: std::collections::HashMap<String, std::collections::HashMap<String, f64>> = std::collections::HashMap::new(); // Initializing HashMap to store exchange rates.

    loop {
        println!("------------------------------- MENU -------------------------------");
        println!("0 - List all available currencies and exchange rates (for US dollar)");
        println!("1 - Enter base currency (the one you convert from)");
        println!("2 - Exit program");
        print!("Enter option: ");
        io::stdout().flush().expect("Failed to flush menu"); // Flushing stdout.

        let mut menu = String::new(); // Initializing string to store menu option.
        io::stdin()
            .read_line(&mut menu)
            .expect("Failed to read menu option"); // Reading menu option.

        let m_value: i32 = menu
            .trim()
            .parse()
            .expect("Input isn't a valid menu option"); // Parsing menu option to integer.

        match m_value {
            0 => {
                display_currencies(&mut curs).await?; // Displaying available currencies and exchange rates.
            }
            1 => {
                read_input_code(&mut curs).await?; // Reading input currency codes and performing conversion.
            }
            2 => {
                println!("Exiting program!"); // Printing exit message.
                break; // Exiting loop.
            }
            _ => {
                println!("Input isn't a valid menu option"); // Printing error message for invalid menu option.
            }
        }
        
        //debug purpose code below, display all cached exchanged rates to optimize and reduce api calls

        // for(key, _value) in &curs {
        //     println!("{:?}", key)
        // }
    }
    Ok(())
}
