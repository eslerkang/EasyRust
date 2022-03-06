// uninitialized variable

fn loop_then_return(mut counter: i32) -> i32 {
  // loop but not rustic
  loop {
    counter += 1;
    if counter % 50 == 0 {
      break;
    }
  }
  counter
}

fn main() {
  let num; // num is uninitialized but not used -> OK
  {
    let x = loop_then_return(32);
    num = x; // num is set by x -> num is num of line 15
  };
  println!("{}", num);
}
