// fn main() {
//     println!("Hello, world!!");
// }

fn main() {
    let my_string = String::from("hello world");

    // first_word가 `String`의 슬라이스로 동작합니다.
    let mut word = first_word(&my_string[..]);
    println!("{}", word);
    my_string.clear();
    println!("{}", word);


}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}