// Wouldn't it be nice to be able to allow functions and other variables to use certain data without fully owning it?
//  This type of functionality is available by using references. References allow us to "borrow" values without taking ownership of them.

let greeting = String::from("hello");
let greeting_reference = &greeting; // We borrow `greeting` but the string data is still owned by `greeting`
println!("Greeting: {}", greeting); // We can still use `greeting`
println!("Greeting reference: {}", greeting_reference); // We can use the reference too

// References in functions

fn print_greeting(message: &String) {
  println!("Greeting: {}", message);
}

fn main() {
  let greeting = String::from("Hello");
  print_greeting(&greeting); // `print_greeting` takes a `&String` not an owned `String` so we borrow `greeting` with `&`
  print_greeting(&greeting); // Since `greeting` didn't move into `print_greeting` we can use it again
}

// However, as we'll see, borrowing a value means we CAN'T do everything we can do with a fully owned value.


// =================================================================================================

// Mutate borrowed values

fn change(message: &String) {
 //  message.push_str("!"); // We try to add a "!" to the end of our message
}

fn main() {
  let greeting = String::from("Hello");
  change(&greeting); 
}

// This code won't compile because we're trying to mutate a borrowed value. Rust doesn't allow this by default.
// We can fix this by using a mutable reference.

fn main() {
  let mut greeting = String::from("hello");
  change(&mut greeting);
}

fn change(text: &mut String) {
  text.push_str(", world");
}

// With & borrows, known as "immutable borrows," we can read the data but we can't change it. With &mut borrows, known as "mutable borrows," we can both read and change the data.

// Your code must implement either of the following definitions, but not both at the same time:

// One or more immutable references (&T)
// Exactly one mutable reference (&mut T)
