fn divide_by_5(num: u32) -> u32 {
  if num == 0 {
    // Return early
    return 0;
  }
  num / 5
}

fn main () {
  let num = 10;
  let result = divide_by_5(num);
  println!("The result of dividing {} by 5 is: {}", num, result);
}