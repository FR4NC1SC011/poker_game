use pickledb::{PickleDb, PickleDbDumpPolicy, SerializationMethod};
use std::io::{self, BufRead};
use std::process;

// --------------------------------------------------------------------------------------
//                  DATABASE
// --------------------------------------------------------------------------------------

// Attempts to load a DB from a file, and if it fails to load, constructs a new `PickleDb` instance.
pub fn load_or_new_db(db_name: &str) -> PickleDb {
    let load = PickleDb::load(
                            db_name,
                            PickleDbDumpPolicy::AutoDump,
                            SerializationMethod::Json);
    return match load {
        Ok(load) => load,
        Err(_) => {
            create_db(db_name)
        }
    };


}

// Create a new DB
pub fn create_db(db_name: &str) -> PickleDb {
    // create a new DB with AutoDump (meaning every change is written to the file)
    // and with Json serialization (meaning DB will be dumped to file as a Json object)
    let mut new_db = PickleDb::new(
        db_name,
        PickleDbDumpPolicy::AutoDump,
        SerializationMethod::Json,
    );

    // set the value 0 to the key 'HighScore'
    new_db.set("HighScore", &0).unwrap();

    // Return DB
    new_db
}

// --------------------------------------------------------------------------------------
//                  MONEY BET
// --------------------------------------------------------------------------------------

// Bet from the user
pub fn money_bet(money: u32) -> u32 {
    // Check if the user has enough money to bet
    if money < 100 {
        println!("No More Money");
        // Exit program
        process::exit(1);
    }

    // Ask user how much money he wants to bet
    let bet: u32 = get_input("Input Bet(min 100)");


    // The bet needs to be a minimum of 100
    // Check if the bet is valid
    if bet < 100 || bet > money {
        println!("Invalid quantity The Bet is 100");
        // If invalid return the minimum bet
        return 100;
    } else {
        println!("Bet is {}", bet);
        return bet;
    }

}

// --------------------------------------------------------------------------------------
//                  USER INPUT
// --------------------------------------------------------------------------------------

// Get input from the user
pub fn get_input<U: std::str::FromStr>(prompt: &str) -> U {

    loop {
        let mut input = String::new();

        // Reads the input from STDIN and places it in the String named input.
        println!("{}", prompt);
        io::stdin().read_line(&mut input)
            .expect("Failed to read input.");

        // Convert to another type.
        // If successful, bind to a new variable named input.
        // If failed, restart the loop.
        let input = match input.trim().parse::<U>() {
            Ok(parsed_input) => parsed_input,
            Err(_) => {
                println!("Invalid Try Again");
                continue
            },
        };
        return input;
    }
}

// Get user input in the form of a vector
pub fn get_vec_input(prompt: &str) -> Vec<i32> {
    let reader = io::stdin();

    println!("{}", prompt);

    let v: Vec<i32> =
        reader.lock()                           // We take a lock of the stdin which lets you work with stdin as a buffered reader
            .lines().next().unwrap().unwrap()   // Read the next line
            .split(' ').map(|s| s.trim())       // Then we split it by spaces and trim resulting chunks from extra whitespace
            .filter(|s| !s.is_empty())          // Remove empty chunks which were left after trimming
            .map(|s| s.parse().unwrap())        // Convert strings to i32
            .collect();                         // Collect the result to a vector

    v
}
