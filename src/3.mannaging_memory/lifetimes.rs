// Languages like C and C++ often have a problem where a pointer points to an item that's already been freed. This problem is known as a "dangling pointer". 
// Fortunately, Rust eliminates this problem. It guarantees that all references always refer to valid items. But, how does it do it?
//Rust's answer to this question is lifetimes. They allow Rust to ensure memory safety without the performance costs of garbage collection.

fn main() {
  let x;
  {
    let y = 42;
    x = &y; // We store a reference to `y` in `x` but `y` is about to be dropped.
  }
  // println!("x: {}", x); // `x` refers to `y` but `y has been dropped!
}


// =================================================================================================

// Lifetimes in functions

// There may be multiple lifetimes. When that occurs, annotate the lifetimes to help the compiler understand which lifetime it will use to ensure the references 
// are valid at runtime.

// For example, consider a function that takes two strings as its input parameters and returns the longest of them:

fn main() {
  let magic1 = String::from("abracadabra!");
  let magic2 = String::from("shazam!");

  let result = longest_word(&magic1, &magic2);
  println!("The longest magic word is {}", result);
}

fn longest_word(x: &String, y: &String) -> &String {
  if x.len() > y.len() {
    x
  } else {
    y
  }
}

// error[E0106]: missing lifetime specifier
//      --> src/main.rs:9:38
//       |
//     9 | fn longest_word(x: &String, y: &String) -> &String {
//       |                    ----        ----        ^ expected named lifetime parameter
//       |
//       = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `x` or `y`
//     help: consider introducing a named lifetime parameter
//       |
//     9 | fn longest_word<'a>(x: &'a String, y: &'a String) -> &'a String {
//       |                ^^^^    ^^^^^^^        ^^^^^^^        ^^^


// The help text says Rust can't tell whether the reference that's being returned refers to x or y. Neither can we. So, to help identify what the reference is, annotate 
// the return type with a generic parameter to represent the lifetime.

// It's possible that lifetimes could be different whenever the function is called. We don't know the concrete lifetimes of the references that will be passed to our 
// longest_word function, and we can't determine if the reference that will be returned will always be a valid one.

// The borrow checker can't determine if the reference will be a valid one either. It doesn't know how the input parameters lifetime relates to the return value's lifetime. 
// This ambiguity is why we need to annotate the lifetimes manually.

// Luckily, the compiler gave us a hint on how to fix this error. We can add generic lifetime parameters to our function signature. These parameters define the relationship 
// between the references so the borrow checker can complete its analysis:

fn longest_word<'a>(x: &'a String, y: &'a String) -> &'a String {
  if x.len() > y.len() {
      x
  } else {
      y
  }
}

// The variables x and y must have the same lifetime. The return value must have the same lifetime as x and y. This is what the <'a> syntax means. It's a generic lifetime

// =================================================================================================

// Annoting lifetimes in types

// Whenever a struct or enum holds a reference in one of its fields, we must annotate that type definition with the lifetime of each reference that it carries along with it.

#[derive(Debug)]
struct Highlight<'document>(&'document str);

fn main() {
  let text = String::from("The quick brown fox jumps over the lazy dog.");
  let fox = Highlight(&text[4..19]);
  let dog = Highlight(&text[35..43]);
  println!("{:?}", fox);
  println!("{:?}", dog);
}