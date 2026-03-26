fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);//borrowing a value
    let mut s = String::from("hello");
    change(&s);
    change2(&mut s); // Can be modified since passed as a mutable reference
    println!("{len}");

    // If we have a mutable reference to a value, you can have no other references to that value and vice versa
    let r1 = &mut s;
    // let r2 = &mut s; This Gives error

    *r1 = String::from("hehehe");
    println!("{r1}");

    // Note that a reference’s scope starts from where it is introduced and continues through the last time that reference is used.

    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{r1} and {r2}");
    // Variables r1 and r2 will not be used after this point.

    let r3 = &mut s; // no problem
    println!("{r3}");
    
    // let reference_to_nothing = dangle();
    let reference_to_something = no_dangle();
    println!("{reference_to_something}");


}

// This is problematic since s becomes a dangling pointer after the function execution since memory in s if freed
// fn dangle() -> &String {
//     let s = String::from("hello");

//     &s
// }

// The sultion is to return the string itself, thereby transferring ownership
fn no_dangle() -> String {
    let s = String::from("hello");

    s
}


fn change(str: &String){
    // str.push_str("hehe"); Cannot modify a borrowed value
    println!("{str}");
}
fn change2(str: &mut String){
    str.push_str("World");
}

fn calculate_length(s: &String) -> usize{
    s.len()
}