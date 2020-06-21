fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    let y = 5;
    println!("The value of y is: {}", y);
    let y = y + 1;
    println!("The value of y is: {}", y);

    let p: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
}
