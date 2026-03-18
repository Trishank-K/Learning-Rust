fn main(){
    let x = 5;
    println!("The value of x is: {x}");
    // x = 6; Does not work
    println!("The value of x is: {x}");
    let mut y = 5; // mut does not allow us to mutate a variable's type; but shadowing does
    println!("The value of x is: {y}");
    y = 6;
    println!("The value of x is: {y}");
    let str = "    ";
    let str = str.len();// Shadowing and changing the type
    println!("The size of str is : {str}");

}