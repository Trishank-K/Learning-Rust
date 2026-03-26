fn main() {
    println!("Hello, world!");
    let str = String::from("He llo");
    first_word(&str);

    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("{hello} {world}");
    let first = first_word_slice(&s);
    println!("{first}")
}

fn first_word(s: &String) -> usize{
    let bytes = s.as_bytes();// convert string to array of bytes
    for i in bytes {
        print!("{i} ");
    }
    for(i, &item) in bytes.iter().enumerate(){ // (index, reference to that element)
        if item == b' ' { // b' ' | b is a byte literal
            return i;
        }
    }
    return bytes.len();
}
fn first_word_slice(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}