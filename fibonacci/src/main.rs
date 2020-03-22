use std::io;
use std::time::Instant;

fn main() {
    println! ("Which position?");
    let mut num = String::new();

    io::stdin().read_line(&mut num).expect("Couldn't read number.");

    let num : i32 = match num.trim().parse(){
        Ok(num) => num,
        Err(_) => 0
    };

    let now = Instant::now();
    println! ("{}", get_fibonacci_number(num));
    println! ("Time taken to calculate: {}", now.elapsed().as_secs());
}

fn get_fibonacci_number (num: i32) -> i64 {
    match num {
        0 => 1,
        1 => 1,
        _ => get_fibonacci_number(num-1) + get_fibonacci_number(num-2)
    }
}
