// Statement are instructions that perform an action and do not return a value.
// Expressions evaluate to a resultant value.
fn main() {
    println!("Hello, world!");
    let y = 6; //statement
    //let z = x = 6; Gives error because x = 6 does not return anything
    another_function();
    println!("{y}");
    print_something(5, 's');

    let z = {
        let x = 3;
        x + 1 // This is an expression. If there were a semicolon in the end it would've been a statement
    };
    println!("{z}");
    let x = five();
    println!("{x}");
    let x = plus_one(x);
    println!("{x}");

}

// Functions implicitly return the last expression
fn five()-> i32{
    5
}

fn plus_one(x: i32) -> i32{
    x+1
    // x+1; This gives an error
}

fn another_function() {
    println!("Another function.");
}

fn print_something(num: i32, text: char){
    println!("{num}, {text}");
}