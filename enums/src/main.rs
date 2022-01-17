fn main() {
    
    // ========= 6.1 enums =========
    enum IpAddrKind {
        V4,
        V6,
    }

    struct IpAddr0 {
        kind: IpAddrKind,
        address: String,
    }

    let home0 = IpAddr0 {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback0 = IpAddr0 {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    //more concise version:
    enum IpAddr1 {
        V4(String),
        V6(String),
    }

    let home1 = IpAddr1::V4(String::from("127.0.0.1"));
    let loopback1 = IpAddr1::V6(String::from("::1"));

    //better version
    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let home = IpA
    let loopback = IpAddr::V6(String::from("::1"));

    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    //enums like structs can have methods
    impl Message {
        fn call(&self) {
            // method body would be defined here
        }
    }

    let m = Message::Write(String::from("hello"));
    m.call();

    //how rust handles null
    let some_number = Some(5);
    let some_string = Some("a string");

    let absent_number: Option<i32> = None;

    //========= 6.2 match control flow operator =========
    #[derive(Debug)] // so we can inspect the state in a minute
    enum UsState {
        Alabama,
        Alaska,
        // --snip--
    }

    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }

    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("State quarter from {:?}!", state);
                25
            }
        }
    }

    fn value_in_cents_penny(coin: Coin) -> u8 {
        //if operator must return bool but match can return anything
        match coin {
            Coin::Penny => {
                println!("Lucky penny!");
                1
            }
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        }
    }

    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    //matches must be exhaustive but rust allows for catch all patterns
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(),
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn reroll() {}

    //6.3 if let syntax
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }

}
