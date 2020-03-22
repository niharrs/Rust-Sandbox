fn main() {
    let tup = (-500, 1, "hello");
    let (x, y, z) = tup;
    println! ("{}", tup.0);
    println! ("{}", tup.1);
    println! ("{}", tup.2);
    println! ("{}", x);
    println! ("{}", y);
    println! ("{}", z);
}
