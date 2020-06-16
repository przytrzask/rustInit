use std::io;

fn main() {
    // variables
let mut name = "Pascal";
    println!("{}",name);
    name = "Alice";
    println!("{} and {}", name, name);

    // functions
    let first = "Tomek".to_string();
    let last = "Trzask".to_string();
    say_name(first,last);

    //borrowing
let first_name = "Tommy".to_string();
    say_first_name(&first_name);
    say_first_name(&first_name);

    println!("please enter your name:");

        let mut input_name = String::new();
    io::stdin().read_line(&mut input_name);
    println!("Your name is {}", input_name);


}

fn say_name(first: String, last: String) {
    println!("{} {}", first,last);
}

fn say_first_name(name: &String) {
    println!("{}", name);
}
