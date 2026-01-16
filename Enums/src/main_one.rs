enum IpAddrKind {
    V4,
    V6
}

fn main () {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    let absent_ip : Option<IpAddrKind> = None;

    let x: i8 = 8;
    let y: Option<i8> = Some(5);

    // let sum = x + y; won't compile because y is an Option type and null is not handled
    let sum = x + y.unwrap_or(0); // unwrap_or provides a default value if y is None    
    println!("Sum: {}", sum);

    valueInCents(Coin::Penny(UsState::Alabama));
}

// using match expression to handle Option
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Arizona,
    Arkansas,
    California,
}

enum Coin {
    Penny (UsState),
    Nickel (UsState),
    Dime (UsState),
    Quarter (UsState),
}

fn valueInCents (coin: Coin) -> u8 {
    match coin {
        Coin::Penny (state) => {
            println!("Lucky penny!{:? }" , state);
            1
        }
        Coin::Nickel (state) => {
            println!("Nickel from the state!{:?}", state);
            5
        }
        Coin::Dime (state) => {
            println!("Dime from!{:?}", state);
            10
        }
        Coin::Quarter (state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}