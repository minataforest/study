use std::io;

fn main(){    
    loop {
        println!("
            1. 섭씨, 화씨 변경
            2. 피보나치 수열
            3. 크리스마스 캐롤
            4. 그만하기
        ");
        println!("원하는 숫자를 입력하세요.");
    
        let mut number = String::new();
    
        io::stdin().read_line(&mut number)
            .expect("Failed to read line");
        
        let number: u32 = number.trim().parse()
            .expect("Fail to int");

        if number == 1 {
            temp();
        } else if number == 2 {
            fivo();
        } else if number == 3 {
            carol();
        } else if number == 4 {
            println!("bye~");
            break;
        } else {
            println!("잘못 입력했습니다.");
        }
    }
}

// 섭씨를 화씨로 바꾸기
fn temp(){
    let mut typ = String::new();
    let mut org_temper = String::new();
    let mut new_temper: f64 = 0.0;
    
    println!("
    1. 섭씨 -> 화씨
    2. 화씨 -> 섭씨
    3. 그만하기
    ");
    
    io::stdin().read_line(&mut typ).expect("Fail to read line");
    
    println!("온도를 입력하세요.");
    io::stdin().read_line(&mut org_temper).expect("Fail to read line");
    
    let org_temper: f64 = org_temper.trim().parse().expect("Fail to float");

    if typ.trim() == "1" {
        new_temper = (org_temper * 9.0/5.0) + 32.0;
        println!("섭씨 {}는 화씨 {}입니다.", org_temper, new_temper);
    } else if typ.trim() == "2" {
        new_temper = (org_temper -32.0) * 5.0/9.0;
        println!("화씨 {}는 섭씨 {}입니다.", org_temper, new_temper);
    } else {
        println!("잘못 입력했습니다.");
    }
}

// n번째 피보나치 수열 생성.
fn fivo(){
    println!("피보");
}
// 크리스마스 캐롤 “The Twelve Days of Christmas”의 가사를 반복문을 활용해 출력.
fn carol(){ 
    let years = ["first", "second", "third", "fourth", "fifth", "six"];
    let texts = ["", "Two turtle-doves", "Three French hens", "Four calling birds", "Five golden rings (five golden rings)", "Six geese a laying"];
    let mut text = String::new();

    for index in 0..6 {
        println!("On the {} day of Christmas", years[index]);
        println!("My true love sent to me");
        if index > 0 {
            text = texts[index].to_owned() + "\n" + &text;
            println!("{}", text);
        } 
        println!("A partridge in a pear tree\n");
    }
    
}


// ==============반복문===============
// fn main(){
//     let a = [10, 20, 30, 40, 50];

//     for element in a.iter() {
//         println!("the value is {}", element);
//     }

//     for number in (1..4).rev(){
//         println!("{}", number);
//     }

//     println!("LIFTOFF!!!");
// }

// fn main(){
//     let mut number = 3;

//     while number != 0 {
//         println!("{}!", number);

//         number -= 1;
//     }

//     println!("LIFTOFF!!!");
// }

// fn main(){
//     loop {
//         println!("again!");
//         break;
//     }
// }


// ============== 조건문 ================
// fn main(){
//     let condition = true;
//     let number = if condition {
//         5
//     } else {
//         "six"
//     }

//     println!("The value of number is: {}", number)

// }


// =============================== 변수, 힘수 =======================
// fn main() {
//     println!("Hello, world!");
//     // another_functions(5, 6); // 5: 전달인자
//     let x = plus_one(5);

//     println!("x: {}", x);
// }

// fn plus_one(x: i32) -> i32 {
//     x + 1
// }

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