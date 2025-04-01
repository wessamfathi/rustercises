fn main() {
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),
    }

    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    }

    let mut _count = 0;
    let coin = Coin::Quarter(State::Alabama);
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {state:?}");
    } else {
        _count += 1;
    }
}

#[derive(Debug)]
enum State {
    Alaska,
    Alabama,
}

enum Coin {
    Penny,
    Cent,
    Quarter(State),
    Dollar,
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
