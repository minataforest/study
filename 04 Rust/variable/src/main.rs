fn main() {
    println!("Hello, world!");
    // another_functions(5, 6); // 5: 전달인자
    let x = plus_one(5);

    println!("x: {}", x);
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

// fn another_functions(x: i32, y: i32){ // X: 매개변수. 매개변수에는 반드시 타입을 정의해야 한다.
//     println!("The value of x is: {}", x);
//     println!("The value of y is: {}", y);
// }

// fn variable(){
//     // mut
//     let mut x = 5;
//     println!("The value of x is: {}", x);
//     x = 6;
//     println!("The value of x is: {}", x);
    
//     // 상수
//     const MAX_POINTS: u32 = 100_000;
//     println!("The value of MAX_POINTS is: {}", MAX_POINTS);

//     // shadow
//     let x = 5;
//     let x = x + 1;
//     let x = x * 2;

//     println!("The value of x is: {}", x);
    
//     // integers
//     let decimal = 98_222;
//     println!("The value of decimal is: {}", decimal);
//     let _hex = 0xff; // 255
//     println!("The value of _hex is: {}", 0xff);
//     let octal = 0o77; // 63
//     println!("The value of octal is: {}", octal);

//     // float
//     let x = 2.0; // f64
//     let y: f32 = 3.0; // f32
//     println!("The value of x, y is: {}, {}", x, y);

//     // char
//     let c = 'z';
//     let z = 'ℤ';
//     let heart_eyed_cat = '😻';
//     println!("The value of c, z, heart_eyed_cat is: {}, {}, {}", c, z, heart_eyed_cat);

//     // tuple
//     let tup = (500, 6.4, 1);
//     let (a, b, c) = tup;
//     println!("The value of b is: {}", b);  // 6.4

//     let x: (i32, f64, u8) = (500, 6.4, 1);
//     let five_hundred = x.0;  // 500
//     let six_point_four = x.1; // 6.4
//     let one = x.2;  // 1
//     println!("The value of tuple is: {}, {}, {}", five_hundred, six_point_four, one);

//     // array
//     let a = [1, 2, 3, 4, 5];
//     let first = a[0];
//     let second = a[1];
//     println!("The value of array is: {}, {}", first, second);
// }