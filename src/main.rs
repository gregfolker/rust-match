// Project: rust-match
// Author: Greg Folker

#[derive(Debug)]
enum Coin {
    _Penny,
    _Nickel,
    _Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
    _Alabama,
    Alaska,
    // ---more states---
}

fn main() {
	println!("Hello, World!");

    let c = Coin::Quarter(UsState::Alaska);
    let v = value_in_cents(c);

    println!("This quarter is worth {} cents", v);

    let _five = Some(5);
    let _six = plus_one(_five);
    let _none = plus_one(None);

    // `_` can be used as a placeholder for the cases you do
    // not want to list out inside a `match` statement
    let byte = 0x07;

    match byte {
        0x01 => println!("one!"),
        0x03 => println!("three!"),
        0x05 => println!("five!"),
        0x07 => println!("seven!"),
        _ => (),
    }

    // A `match` statement can be wordy if you only care about one of the cases
    // In these cases, it may be better to use the `let if` control flow
    let some_byte = Some(0x08);

    if let Some(3) = some_byte {
        println!("three!");
    } else {
        // An else can be used if you want to handle the negative cases as well
        println!("not three!");
    }
}

fn value_in_cents(coin: Coin) -> u8 {
    // The `match` statement in Rust is essentially just a switch
    match coin {
        Coin::_Penny => 1,
        Coin::_Nickel => 5,
        Coin::_Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

// `match` can also be used to operate on a variety of types using `Option<T>`
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        // The `None` case is required
        None => None,
        Some(i) => Some(i + 1),
    }
}
