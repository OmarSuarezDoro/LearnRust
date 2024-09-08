// TODO: modify only this function.

// We need to specify that the lifetime of the reference returned by the function is the same as the lifetime of the references passed to the function.
// In this case the lifetime of the vector.
fn copy_and_return<'a>(vector: &'a mut Vec<String>, value: &str) -> &'a str {
  vector.push(String::from(value));
  return &vector[vector.len() - 1];
}

fn main() {
  let name1 = "Joe";
  let name2 = "Chris";
  let name3 = "Anne";

  let mut names = Vec::new();

  assert_eq!("Joe", copy_and_return(&mut names, &name1));
  assert_eq!("Chris", copy_and_return(&mut names, &name2));
  assert_eq!("Anne", copy_and_return(&mut names, &name3));

  assert_eq!(
    names,
    vec!["Joe".to_string(), "Chris".to_string(), "Anne".to_string()]
  );

  println!("Exercise complete! You implemented the `copy_and_return` function correctly!");
}