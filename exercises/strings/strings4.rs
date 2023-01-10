// strings4.rs

// Ok, here are a bunch of values-- some are `String`s, some are `&str`s. Your
// task is to call one of these two functions on each value depending on what
// you think each value is. That is, add either `string_slice` or `string`
// before the parentheses on each line. If you're right, it will compile!
// No hints this time!

// I AM NOT DONE

fn string_slice(arg: &str) {
    println!("{}", arg);
}
fn string(arg: String) {
    println!("{}", arg);
}

fn main() {
    let mut owned_string: String = "rust is fun!".to_owned();
    let borrowed_string: &str = "bar";
    // owned_string.push_str(borrowed_string);
    // string(owned_string);

    // let new_string = format!("{}{}", owned_string, borrowed_string);
    // let new_string = format!("{}{}", new_string, borrowed_string);

    for _ in 0..5 {
        owned_string.push_str("bar");
    }


    let together = owned_string.clone() + borrowed_string;
    println!("{}", together);
    // string(owned_string);

    // string(format!("{}{}", owned_string, borrowed_string))

    // string_slice("blue");
    // string("red".to_string());
    // string(String::from("hi"));
    // string_slice("nice weather".into());
    // string(format!("Interpolation {}", "Station"));
    // string_slice(&String::from("abc")[0..1]);
    // string_slice("  hello there ".trim());
    // string("Happy Monday!".to_string().replace("Mon", "Tues"));
    // string("mY sHiFt KeY iS sTiCkY".to_lowercase());
}
