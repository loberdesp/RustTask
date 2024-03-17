use task::converter_module::*;
use std::io::Write; // Importing Write trait for flushing output.

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> { 
    let mut curs: std::collections::HashMap<String, std::collections::HashMap<String, f64>> = std::collections::HashMap::new(); // Initializing HashMap to store exchange rates.

    loop {
        println!("");
        println!("");
        println!("------------------------------- MENU -------------------------------");
        println!("0 - List all available currencies and exchange rates (for US dollar)");
        println!("1 - Enter base currency (the one you convert from)");
        println!("2 - Exit program");
        print!("Enter option: ");
        std::io::stdout().flush().expect("Failed to flush menu");

        let mut menu = String::new(); // Initializing string to store menu option.
        std::io::stdin()
            .read_line(&mut menu)
            .expect("Failed to read menu option");

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
                println!("");
                println!("Exiting program!");
                println!("");
                break;
            }
            _ => {
                println!("");
                println!("Input isn't a valid menu option"); // Printing error message for invalid menu option.
            }
        }
    }
    Ok(())
}