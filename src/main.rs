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


async fn display_currencies(m: &mut Option<std::collections::HashMap<String, std::collections::HashMap<String, f64>>>) -> Result<(), reqwest::Error> {
    let list_link = "https://v6.exchangerate-api.com/v6/a3f798577a713b0309d32d40/latest/USD";
        let list_response = reqwest::Client::new().get(list_link).send().await;

        


        match list_response {
            Ok(res) => {
                if res.status().is_success() {
                    let list_body: ExchangeRates = res.json().await?;

                    if let Some(map) = m {
                        map.insert("USD".to_string(), list_body.conversion_rates.clone());
                    }


                    for (key, value) in list_body.conversion_rates {
                        println!("Code: {}, Rate: {}", key, value);
                    }
                }
            }
            Err(err) => {
                eprintln!("Failed to parse currencies: {}", err);
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
        .expect("Input not an integer");

    if value<=0.0 {
        println!("Value less or equal to 0, enter valid value");
        return -1.0;
    } else {
        return value;
    } 
}

fn is_uppercase(input: &str) -> bool {
    input.chars().all(char::is_uppercase)
}

async fn read_input_code(available_currencies: &mut Option<std::collections::HashMap<String, std::collections::HashMap<String, f64>>>) -> Result<(), reqwest::Error> {

    let mut cur_from = String::new();
    loop {
        print!("Enter base currency (e.g., USD): ");
        io::stdout().flush().expect("Failed to flush base currency");
        io::stdin().read_line(&mut cur_from).expect("Failed to read input currency");
        cur_from = cur_from.trim().to_string();

        if is_uppercase(&cur_from) {
            break;
        } else {
            println!("Invalid input. It should only contain upper case characters!");
            cur_from.clear();
        }
    }
    

    let mut cur_to = String::new();
    loop {
        print!("Enter output currency (e.g., GBP): ");
        io::stdout().flush().expect("Failed to flush output currency");
        io::stdin().read_line(&mut cur_to).expect("Failed to read output currency");
        cur_to = cur_to.trim().to_string();

        if is_uppercase(&cur_to) {
            break;
        } else {
            println!("Invalid input. It should only contain upper case characters!");
            cur_to.clear();
        }
    }
    
    if let Some(currency_rates) = available_currencies {

        // for (key, _value) in currency_rates.clone() {
        //     println!("Key: {}", key);
        // }

        //nie dodaje sie klucz

        // if currency_rates.contains_key(&cur_from) {
        //     println!("JEST");
        // } else {
        //     println!("nie ma :(");
        // }

        let num = read_value();
        if num != -1.0 {
            if let Some(rate_from) = currency_rates.get(&cur_from) {
                if let Some(rate_to) = rate_from.get(&cur_to) {
                    non_api_convert(cur_from.to_string(), cur_to.to_string(), num, *rate_to);
                    println!("got it without api!");
                } else {
                    api_convert(cur_from.to_string(), cur_to.to_string(), num, currency_rates).await?;
                    println!("got it with api 1 :(");
                }
            } else {
                api_convert(cur_from.to_string(), cur_to.to_string(), num, currency_rates).await?;
                println!("got it with api 2 :(");
            }
        }
    }
    Ok(())
}


fn non_api_convert(from: String, to: String, amount: f64, rate: f64) {
    println!("{:.2} {} exchanged with {} rate is {:.2} {}", amount, &from, rate, amount*rate, &to);
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

                    // if let Some(inner_map) = map.as_ref().and_then(|m| m.get(&from)) {
                    //     println!("inside, no need to add, can call non_api function");

                    // } else {
                    //     println!("not inside, gotta add it to the map");
                    //     let mut new_map: std::collections::HashMap<String, std::collections::HashMap<String, f64>> = std::collections::HashMap::new();
                    //     map.insert(new_map);
                    // }

                    
                    let body: ExchangeRates = res.json().await?;


                    let _ = map.insert(from.clone(), body.conversion_rates.clone());

                    // println!("Inserted {} exchange hashmap", from.clone());

                    
                    // if let Some(m) = map.as_mut() {
                    //     for (key, _inner_map) in m {
                    //         println!("Outer Key: {}", key);
                    //     }
                    // }



                    

                    
                    

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

    let mut curs: Option<std::collections::HashMap<String, std::collections::HashMap<String, f64>>> = Some(std::collections::HashMap::new());

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
                //curs = Some(display_currencies().await?);
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
        
        if let Some(m) = &curs {
            for (key, _inner_map) in m {
                println!("Keys after one loop: {}", key);
            }
        }
    }
    Ok(())
}