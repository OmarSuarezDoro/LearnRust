fn main() {
  let mut i = 0;
  // Infinite loop, break when i == 10
  loop {
    i += 1;
    if i == 10 {
      break;
    }
  }
  println!("i = {}", i);

  let mut j = 0;
  while j < 10 {
      j += 1;
  }
  println!("j = {}", j);
  // reverse loop
  for k in (0..10).rev() {
    println!("k = {}", k);
  }
}