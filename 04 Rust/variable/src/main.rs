fn main() {
    // mut
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
    
    // 상수
    const MAX_POINTS: u32 = 100_000;
    println!("The value of MAX_POINTS is: {}", MAX_POINTS);

    // shadow
    let x = 5;
    let x = x + 1;
    let x = x * 2;

    println!("The value of x is: {}", x);
    
    let decimal = 98_222;
    println!("The value of decimal is: {}", decimal);
    let _hex = 0xff; // 255
    println!("The value of _hex is: {}", 0xff);
    let octal = 0o77; // 63
    println!("The value of octal is: {}", octal);

}
