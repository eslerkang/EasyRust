// option - maybe there, maybe not
// result

// enum Option<T> {
//   None,
//   Some(T)
// }
// .is_none()
// .is_some()
// None.unwrap() => panic

// enum Result<T, E> {
//   Ok(T),
//   Err(E)
// }
// .is_ok()
// .is_err()
// Err.unwrap() => panic

fn check_error(input: i32) -> Result<(), ()> {
  if input % 2 == 0 {
    Ok(())
  } else {
    Err(())
  }
}

fn check_if_five(number: i32) -> Result<i32, String> {
  match number {
    5 => Ok(number),
    _ => Err(String::from("Not Five"))
  }
}

use std::num::ParseIntError;

// anyhow - crate - generic error

fn parse(number: &str) -> Result<i32, ParseIntError> {
  number.parse()
}

fn main() {
  match check_error(34) {
    Ok(_) => println!("Okay!!"),
    Err(_) => println!("Error!")
  }
  // println!("{:?}", check_error(3).unwrap());

  println!("{:?}", check_if_five(4));

  let mut result_vec = Vec::new();

  for number in 2..=7 {
    result_vec.push(check_if_five(number));
  }

  println!("{:#?}", result_vec);

  let mut vec = vec![];
  vec.push(parse("8"));
  vec.push(parse("a"));
  println!("{:#?}", vec);
}
