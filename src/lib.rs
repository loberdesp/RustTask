pub mod converter_module {
    use reqwest; // Importing the reqwest crate for making HTTP requests.
    use serde::{Serialize, Deserialize}; // Importing serde for serialization and deserialization.
    use std::io; // Importing std::io module for input and output operations.
    use std::io::Write; // Importing Write trait for flushing output.
    use std::time::Duration; // Importing Duration for specifying time durations.
    use tokio::time::sleep; // Importing sleep function from tokio for asynchronous sleep.

    static API_KEY: &str = "a3f798577a713b0309d32d40";

    #[derive(Debug, Serialize, Deserialize)]
    pub struct ExchangeRates {
        pub conversion_rates: std::collections::HashMap<String, f64>, // Struct representing exchange rates.
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct ErrorResponse {
        pub result: String,
        #[serde(rename = "error-type")] //Struct representing error response
        pub error_type: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct SupportedList {
        pub supported_codes: Vec<Vec<String>>, // Struct representing supported currency codes.
    }

    // Function to display available currencies and their exchange rates.
    pub async fn display_currencies(map: &mut std::collections::HashMap<String, std::collections::HashMap<String, f64>>) -> Result<(), reqwest::Error> {
        if let Some(usd_map) = map.get("USD") { // Checking if USD exchange rates are already available.
            for (key, value) in usd_map {
                println!("Code: {}, Rate: 1 USD is {} {}", key, value, key); // Printing currency code and exchange rate.
            }
        } else {
            let base = "https://v6.exchangerate-api.com/v6/".to_string(); // API endpoint for getting currency exchange rates.
            let end = "/latest/USD".to_string();

            let list_link = base + &API_KEY + &end;

            let list_response = reqwest::Client::new().get(&list_link).send().await; // Sending HTTP GET request to get exchange rates.
            match list_response {
                Ok(res) => {
                    if res.status().is_success() { // Checking if the request was successful.
                        let list_body: ExchangeRates = res.json().await?; // Parsing JSON response into ExchangeRates struct.
                        map.insert("USD".to_string(), list_body.conversion_rates.clone()); // Inserting USD exchange rates into the map.
                        for (key, value) in list_body.conversion_rates {
                            println!("Code: {}, Rate: 1 USD is {} {}", key, value, key); // Printing currency code and exchange rate.
                        }
                    }
                }
                Err(err) => {
                    println!("");
                    eprintln!("Failed to parse currencies: {}", err);
                }
            }
        }
        Ok(())
    }


    // Function to read a floating-point value from the user.
    pub fn read_value() -> f64 {
        print!("Value to be converted (e.g., 14.26): ");
        io::stdout().flush().expect("Failed to flush");
        let mut input_line = String::new(); // Creating a new string to store user input.
        io::stdin()
            .read_line(&mut input_line)
            .expect("Failed to read line"); // Reading user input.
        let m = validate_read_value(input_line);
        if m > 0.0 {
            return m;
        } else {
            return -1.0;
        }
    }

    pub fn validate_read_value(input: String) -> f64 {
        let value: f64 = input
            .trim()
            .parse()
            .expect("Input not a valid floating-point number"); // Parsing user input to a floating-point number.
        if value <= 0.0 {
            println!("");
            println!("Value less than or equal to 0, enter a valid value");
            return -1.0;
        } else {
            return value;
        }
    }

    pub fn is_uppercase(input: &str) -> bool {
        input.chars().all(char::is_uppercase) // Checking if all characters in the string are uppercase.
    }

    pub async fn read_input_code(available_currencies: &mut std::collections::HashMap<String, std::collections::HashMap<String, f64>>) -> Result<(), reqwest::Error> {
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
                println!("");
                println!("Invalid input. It should only contain uppercase characters!"); // Printing error message for invalid input.
                println!("");
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
                println!("");
                println!("Invalid input. It should only contain uppercase characters!"); // Printing error message for invalid input.
                println!("");
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

    // Function to perform conversion without using API.
    pub fn non_api_convert(from: String, to: String, amount: f64, rate: f64) {
        println!("");
        println!("");
        println!("#####################################################################");
        println!("{:.2} {} exchanged with {} rate is {:.2} {}", amount, &from, rate, amount * rate, &to);
        println!("#####################################################################");


    }

    pub async fn api_convert(from: String, to: String, amount: f64, map: &mut std::collections::HashMap<String, std::collections::HashMap<String, f64>>) -> Result<(), reqwest::Error> {
        // Function to perform conversion using API.
        let base = "https://v6.exchangerate-api.com/v6/".to_string();
        let end = "/latest/".to_string();

        let link = base + &API_KEY + &end + &from; // Creating API endpoint URL.

        let mut retry_counter = 0; // Initializing retry counter.

        loop {
            let response = reqwest::Client::new().get(&link).send().await; // Sending HTTP GET request to API.

            match response {
                Ok(res) => {
                    if res.status().is_success() { // Checking if the request was successful.
                        let body: ExchangeRates = res.json().await?; // Parsing JSON response into ExchangeRates struct.
                        let _ = map.insert(from.clone(), body.conversion_rates.clone()); // Inserting exchange rates into the map.
                        if let Some(rate) = body.conversion_rates.get(&to) { // Checking if exchange rate for output currency is available.
                            println!("");
                            println!("");
                            println!("#####################################################################");
                            println!("{:.2} {} exchanged with {} rate is {:.2} {}", amount, &from, rate, amount * rate, &to); // Printing conversion result.
                            println!("#####################################################################");


                            break; // Exiting loop.
                        } else {
                            println!("");
                            println!("Error: Invalid output currency: {}", &to);
                            println!("");
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
                        println!("");
                        let error_body: ErrorResponse = res.json().await?; // Parsing JSON error response into ErrorResponse struct.
                        match error_body.error_type.as_ref() { // Matching error type.
                            "unsupported-code" => println!("Error: Invalid input currency code: {}", &from),
                            "malformed-request" => println!("Error: Invalid request structure"),
                            "invalid-key" => println!("Error: Invalid API key"),
                            "inactive-account" => println!("Error: Inactive account (email address not confirmed)"),
                            "quota-reached" => println!("Error: Limit of account's requests exceeded"),
                            _ => println!("Error: Unknown error code"),
                        }
                        println!("");
                        return Ok(());
                    }
                }
                Err(_err) => {
                    println!("");
                    eprintln!("Error: Network error");
                    return Ok(());
                }
            }
        }
        Ok(())
    }
}