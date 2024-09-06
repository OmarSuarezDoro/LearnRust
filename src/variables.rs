fn main() {
  let a_number = 10;
  println!("The value of a_number is: {}", a_number);
  // We can't change the value of a_number because it's immutable
  // Lets try to declare b_number and make it mutable
  let mut b_number = 20;
  println!("The value of b_number is: {}", b_number);
  // Now we can change the value of b_number
  b_number = 30;
  println!("The value of b_number is: {}", b_number);

  // Lets try to see what is variable shadowing
  
  // Declare first variable binding with name "shadow_num"
  let shadow_num = 5;
  // Declare second variable binding, shadows existing variable "shadow_num" 
  let shadow_num = shadow_num + 5;
  // Declare third variable binding, shadows second binding of variable "shadow_num"
  let shadow_num = shadow_num * 2; 

  println!("The number is {}.", shadow_num);


  // Lets type some code to see how does it works
  
  // ======================== INTERGER TYPES ========================
  // Variable as a 32-bit integer
  // The u in u32 stands for unsigned and if we use i32 it will be signed integer
  // We need to keep in mind that rust allow types of intergers of 8, 16, 32, 64, 128 bits
  // Then isize and usize depend on the architecture of the computer
  // For example, if we are using 64-bit architecture, isize and usize will be 64 bits
  // If we are using 32-bit architecture, isize and usize will be 32 bits
  let x : u32 = 5;
  println!("The value of x is: {}", x);

  // ======================== FLOATING POINT TYPES ========================
  // Rust has two types of floating point numbers: f32 and f64
  // f32 is a single-precision float and f64 is a double-precision float
  let y : f32 = 3.0;
  println!("The value of y is: {}", y);
  
  // ======================== BOOLEAN TYPES ========================
  // Rust has a boolean type, bool, which can have two values: true and false
  let is_true : bool = true;
  println!("The value of is_true is: {}", is_true);

  // ======================== CHARACTER TYPES ========================
  // Rust has a char type to represent a single Unicode character
  let character : char = '😂';
  println!("The value of character is: {}", character);
 
  // ======================== STRING TYPE ========================
  // Rust has a string type, str, which is a sequence of Unicode scalar values encoded as a UTF-8 byte sequence
  // Rust also has a string slice type, &str, which is a reference to a string

  // This is a pointer to a string in memory that we can access
  let string : &str = "Hello, world!";
  println!("The value of string is: {}", string);
  // With the type String, we can create a mutable, growable piece of text
  let string : String = String::from("This string is mutable and growable");
  println!("The value of string is: {}", string);



  // ======================== COMPOUND TYPES ========================
  // Rust has two primitive compound types: tuples and arrays
  // Tuples have a fixed length: once declared, they cannot grow or shrink in size
  // Tuples are useful when you want to combine a number of values into one compound value
  // Arrays are useful when you want to store a collection of values of the same type
  // Tuples can have elements of different types
  // Arrays must have elements of the same type
  // Tuples can be destructured to create bindings for each element
  // Arrays can be indexed to access individual elements
  let tuple : (i32, f64, u8) = (500, 6.4, 1);
  let (a, b, c) = tuple;
  println!("The value of a is: {}", a);
  println!("The value of b is: {}", b);
  println!("The value of c is: {}", c);

  // We can also access a tuple element directly by using a period (.) followed by the index of the value we want to access
  let five_hundred = tuple.0;
  let six_point_four = tuple.1;
  let one = tuple.2;
  println!("The value of five_hundred is: {}", five_hundred);
  println!("The value of six_point_four is: {}", six_point_four);
  println!("The value of one is: {}", one);

  // Arrays in Rust have a fixed length, like tuples
  // Arrays are useful when you want your data allocated on the stack rather than the heap
  // Arrays are useful when you want to have a collection of elements of the same type
  // Arrays are useful when you want to iterate over the elements of the array
  // Arrays are useful when you want to access elements by index

  let array = [1, 2, 3, 4, 5];
  println!("The value of array is: {:?}", array);

  // We can access an element of an array by using a period (.) followed by the index of the value we want to access
  let first = array[0];
  let second = array[1];
  println!("The value of first is: {}", first);
  println!("The value of second is: {}", second);
  

  // ======================== STRUCT TYPE ========================
  // Rust has a struct type to define custom data types
  // Structs are useful when you want to group related data together
  // Structs are useful when you want to create a new type that has its own fields
  // Structs are useful when you want to implement methods on a type
  // Structs are useful when you want to implement traits on a type
  // Rust supports three struct types: classic structs, tuple structs, and unit structs. These struct types support different ways to group and work with the data.
  // Classic structs are useful when you want to name each field in the struct
  // Tuple structs are useful when you want to name the struct itself, but not the fields
  // Unit structs are useful when you want to create a type that has no fields

  // Define a struct named "Person" with two fields: name and age
  struct Person {
    name: String,
    age: u8,
  }

  // Define a struct named "Unit" with no fields
  struct Unit;

  // Define a struct named "Tuple" with one field
  struct Tuple(i32);

  // Define a struct named "Classic" with three fields
  struct Classic {
    field1: i32,
    field2: f64,
    field3: u8,
  }


  // Create an instance of the "Person" struct
  let person : Person = Person {
    name: String::from("Alice"),
    age: 30,
  };

  // Create an instance of the "Unit" struct
  let _unit : Unit = Unit;

  // Create an instance of the "Tuple" struct
  let tuple : Tuple = Tuple(5);

  // Create an instance of the "Classic" struct
  let classic : Classic = Classic {
    field1: 500,
    field2: 6.4,
    field3: 1,
  };

  // Access the fields of the "Person" struct
  println!("The name of the person is: {}", person.name);
  println!("The age of the person is: {}", person.age);

  // Access the fields of the "Tuple" struct
  println!("The value of the tuple is: {}", tuple.0);

  // Access the fields of the "Classic" struct
  println!("The value of field1 is: {}", classic.field1);
  println!("The value of field2 is: {}", classic.field2);
  println!("The value of field3 is: {}", classic.field3);

}