use std::io::{self, Write};

#[derive(Debug)] /* Allow us to inspect enums */

enum UsState {
    Alaska,
    Alabama,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents1(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        /* (_) indicates no value for an enum that can take a value  */
        Coin::Quarter(_) => 25,
    }
}

fn value_in_cents2(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky Penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        /* (_) indicates no value for an enum that can take a value  */
        Coin::Quarter(_) => 25,
    }
}

fn value_in_cents3(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10, 
        /* if matched, the value for the Coin::Quarter variant will be bound to the `state` variable */
        Coin::Quarter(state) => {
            println!("State quarter from: {:?}", state);
            25
        }
    }
}

fn move_player(num_spaces : u8) {
    println!("Player moved {} spaces", num_spaces);
}

fn main() {
    let penny : Coin = Coin::Penny;
    let nickel : Coin = Coin::Nickel;
    let r1 = value_in_cents2(penny);
    println!("{}", value_in_cents1(nickel));
    println!("{}", value_in_cents3(Coin::Quarter(UsState::Alaska)));

    /* catch all patterns */
    let dice_roll = 9;
    match dice_roll {
        3 => println!("The roll was a 3"),
        other => move_player(other),
    }
}
