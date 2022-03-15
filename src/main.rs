use std::fmt::Display;
use std::cmp::PartialOrd;

// generics - concrete
// generic - 일반적인 / 대중적인

struct Book {
  year: u16
}

fn give_thing<T: Display>(input: T) -> T { // T
  println!("{}", input); // Display
  input
}

fn compare_and_print<
  T: Display, 
  U: Display + PartialOrd
  >(statement: T, num1: U, num2: U) {
  println!("{}! Is {} greater than {}? => {}", statement, num1, num2, num1 > num2);
}

fn main() {
  let x = give_thing(33);
  let y = give_thing(String::from("haha"));
  println!("{} {}", x, y);
  // let book = give_thing(Book{year: 11}); // cannot be executed

  compare_and_print("Listen up!", 9, 8);
}