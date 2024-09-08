struct Person {
  first: String,
  middle: Option<String>,
  last: String,
}

// match Person.middle(middltext) {
//   Some(middltext) => println!("Middle name: {}", middltext),
//   None => println!("No middle name provided"),
// }

fn build_full_name(person: &Person) -> String {
  let mut full_name = String::new();
  full_name.push_str(&person.first);
  full_name.push_str(" ");

  // TODO: Implement the part of this function that handles the person's middle name.
  match &person.middle {
    Some(middltext) => {
      full_name.push_str(&middltext);
      full_name.push_str(" ");
    },
    None => (),
  }

  full_name.push_str(&person.last);
  full_name
}

fn main() {
  let john = Person {
    first: String::from("James"),
    middle: Some(String::from("Oliver")),
    last: String::from("Smith"),
  };
  assert_eq!(build_full_name(&john), "James Oliver Smith");

  let alice = Person {
    first: String::from("Alice"),
    middle: None,
    last: String::from("Stevens"),
  };
  assert_eq!(build_full_name(&alice), "Alice Stevens");

  let bob = Person {
    first: String::from("Robert"),
    middle: Some(String::from("Murdock")),
    last: String::from("Jones"),
  };
  assert_eq!(build_full_name(&bob), "Robert Murdock Jones");
}