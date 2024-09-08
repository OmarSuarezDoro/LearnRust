// The ownership system:
// Rust includes an ownership system to manage memory. At compile time, the ownership 
// system checks a set of rules to ensure that the ownership features allow your program 
// to run without slowing down.

//// =================================================================================================

// Example of scope

// `mascot` is not valid and cannot be used here, because it's not yet declared.
{
  let mascot = String::from("ferris");   // `mascot` is valid from this point forward.
  // do stuff with `mascot`.
}
// this scope is now over, so `mascot` is no longer valid and cannot be used.



//// =================================================================================================

// Rust adds a twist to the idea of scopes. Whenever an object goes out of scope, it's "dropped." Dropping a variable 
// releases any resources that are tied to it. For example, a file variable resources ends up when the file is closed. 
// For variables that have allocated memory associated with them, the memory is freed.

// In Rust, bindings that have things "associated" with them that they'll free when the binding is dropped are said to "own" those things.

// In the previous example, the mascot variable owns the String data associated with it. The String itself owns the heap-allocated memory 
// that holds the characters of that string. At the end of the scope, mascot is "dropped", the String it owns is dropped, and finally the 
// memory that String owns is freed.




// Sometimes, though, we don't want the things associated with a variable to be dropped at the end of scope. 
// Instead, we want to transfer ownership of an item from one binding to another.


//// =================================================================================================

// The simplest example is when declaring a new binding:
{
  let mascot = String::from("ferris");
  // transfer ownership of mascot to the variable ferris.
  let ferris = mascot;
  println!("{}", ferris);
}
// ferris is dropped here. The string data memory will be freed here.

//// =================================================================================================


// A key thing to understand is that once ownership is transferred, the old variable is no longer valid. In our 
// previous example, after we transfer ownership of the String from mascot to ferris, we can no longer use the mascot variable.

// In Rust, "transferring ownership" is known as "moving". In other words, the ownership of the String value has been moved from mascot to ferris.
// If we try to use mascot after the String has been moved from mascot to ferris, the compiler won't compile our code.


// Let's take a look at an example of a string being passed to a function as an argument. Passing something as an argument to a function moves that thing into the function.

fn process(input: String) {}

fn caller() {
  let s = String::from("Hello, world!");
  process(s); // Ownership of the string in `s` moved into `process`
  // process(s); // Error! ownership already moved.
}

// In other programming languages, the String value of the s variable can be implicitly copied before being passed to our function. But in Rust, 
//this action doesn't happen.
// In Rust, ownership transfer (that is, moving) is the default behavior.

// =================================================================================================

// Copying instead of moving

// Let's take a look at a value that implements the Copy trait: u32. The following code mirrors our broken code, but it compiles without issue.

fn process(input: u32) {}

fn caller() {
  let n = 1u32;
  process(n); // Ownership of the number in `n` copied into `process`
  process(n); // `n` can be used again because it wasn't moved, it was copied.
}

// Simple types like numbers are copy types. They implement the Copy trait, which means they're copied rather than moved. The same action occurs for 
// most simple types. Copying numbers is inexpensive, so it makes sense for these values to be copied. Copying strings or vectors or other complex 
// types can be expensive, so they don't implement the Copy trait and are instead moved.


// How do we copy types that don't implement Copy?

// A call to .clone duplicates the memory and produce a new value. The new value is moved meaning the old value can still be used.

fn process(s: String) {}

fn main() {
  let s = String::from("Hello, world!");
  process(s.clone()); // Passing another value, cloned from `s`.
  process(s); // s was never moved and so it can still be used.
}

// This approach can be useful, but it can make your code slower as every call to clone makes a full copy of the data. This method often includes memory 
// allocations or other expensive operations. We can avoid these costs if we "borrow" values by using references.