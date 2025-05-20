fn main() {
    let is_true = true;

    if is_true {
        println!("TRUE")
    } else {
        println!("FALSE")
    }

    for i in 0..11 {
        print!("{} ", i);
    }
    println!("");
    let sentence: String = String::from("My Name is Trishank!");
    let first_word: String = get_first_word(sentence);
    println!("First Word is: {}", first_word);
}

fn get_first_word(sentence: String) -> String {
    let mut ans: String = String::from("");
    for char in sentence.chars() {
        ans.push_str(char.to_string().as_str());
        if char == ' ' {
            break;
        }
    }
    return ans;
}
