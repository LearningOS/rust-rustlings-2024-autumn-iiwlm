// strings4.rs
//
// Ok, here are a bunch of values-- some are `String`s, some are `&str`s. Your
// task is to call one of these two functions on each value depending on what
// you think each value is. That is, add either `string_slice` or `string`
// before the parentheses on each line. If you're right, it will compile!
//
// No hints this time!


fn string_slice(arg: &str) {
    println!("{}", arg);
}
fn string(arg: String) {
    println!("{}", arg);
}

fn main() {
    let blue:&str="blue";
    let red="red".to_string();
    let hi = String::from("hi");
    let fun ="rust is fun!".to_owned();
    let nice="nice weather".to_string();
    let Station = format!("Interpolation {}", "Station");
    let abc = &String::from("abc")[0..1];
    let hello="  hello there ".trim();
    let Happy="Happy Monday!".to_string().replace("Mon", "Tues");
    let mY = "mY sHiFt KeY iS sTiCkY".to_lowercase();
}
