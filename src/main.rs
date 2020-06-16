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

}

fn say_name(first: String, last: String) {

    println!("{} {}", first,last);
}
