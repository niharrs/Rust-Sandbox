use std::io;

fn main() {
    let mut temp = String::new();
    let mut choice = String::new();

    println! ("Press 1 to convert temperate from C to F");
    println! ("Press 2 to convert temperate from F to C");

    io::stdin().read_line(&mut choice).expect("Failed to read the choice.");

    //Convert string to int  
    let choice : i8 = match choice.trim().parse(){
        Ok(num) => num,
        Err(_) => 0
    };

    println! ("Enter the temperature.");

    io::stdin().read_line(&mut temp).expect("Failed to read the temperature.");

    //Convert string to float
    let temp : f64 = match temp.trim().parse(){
        Ok(num) => num,
        Err(_) => 0.0
    };

    if choice == 2 {
        println!("{}", convert_fahrenheit_to_celcius(temp));
    } else {
        println! ("{}", convert_celcius_to_fahrenheit(temp));
    }
}

fn convert_fahrenheit_to_celcius(temp: f64) -> f64 {
    //Deduct 32, then multiply by 5, then divide by 9
    ((temp-32.0)*5.0)/9.0
}

fn convert_celcius_to_fahrenheit(temp: f64) -> f64 {
    //Divide by 5, then multiply by 9, then add 32
    ((temp/5.0)*9.0)+32.0
}