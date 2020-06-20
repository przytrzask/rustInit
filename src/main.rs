use std::io;
use std::process;


fn main() {
 
let mut name = "Pascal";


    loop{
    println!("Please enter a first number:");
  let a = read_user_input();

  println!("Please enter a second number:");
        let b = read_user_input();
     let result = sum(a, b);
     println!("{} + {} = {}", a, b, result);
};


}


fn read_user_input() ->u32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
  
    let digit:u32;
  
    match input.trim().parse() {
      Ok(val) => digit = val,
      Err(_err) => {
        println!("Not a valid number!");
        process::exit(1);
      }
    };
  
    digit


}

fn say_name(first: String, last: String) {
    println!("{} {}", first,last);
}

fn say_first_name(name: &String) {
    println!("{}", name);
}
fn sum(a: u32, b: u32) -> u32 {
    a + b
}



//     println!("{}",name);
//     name = "Alice";
//     println!("{} and {}", name, name);

//     // functions
//     let first = "Tomek".to_string();
//     let last = "Trzask".to_string();
//     say_name(first,last);

//     //borrowing
// let first_name = "Tommy".to_string();
//     say_first_name(&first_name);
//     say_first_name(&first_name);

//     println!("please enter your name:");

    // read input
    //     let mut input_name = String::new();
    // io::stdin().read_line(&mut input_name);
    // println!("Your name is {}", input_name);

     // unwrap
    // let a:u32 = first.trim().parse().unwrap();
    // expect
    // let a:u32 = first.trim().parse().expect("It's not a valid number");

    //pattern matching