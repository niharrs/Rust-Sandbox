#[derive(Debug)]
enum Person {
    Name(String),
    Age(i32)
}

fn main() {
    let mut v1 : Vec <i32> = Vec::new();
    let mut v2 = vec![1, 2, 3];

    v1.push(4);
    v1.push(5);
    v1.push(6);

    //check if 3rd index is present in v2
    let third : &i32 = &v2[2];
    println! ("Third elemement = {}", third);

    match v2.get(2) { // get returns Option<&T>, doesn't panick if we try to fetch a value larger than length of v2
        Some(third) => println! ("Third element is present."),
        None => println! ("Third element is not present.")
    }

    for i in &mut v2 {
        *i += 50;
    }
    println! ("{:#?}", v2);

    let friend = vec! [
        Person::Name(String::from("Niharika")),
        Person::Age(67)
    ];

    println! ("{:#?}", friend);
}

//Here: v1 and v2 are out of scope