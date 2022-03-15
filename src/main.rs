// Option
// Result
// from 'OCaml' language
// rust dont have null -> None
// rust panic == 안될 것 같다(포기)

// Option T

fn take_fifth(value: Vec<i32>) -> Option<i32> {
  if value.len() < 5 {
    None // Option::None
  } else {
    Some(value[4]) // i32
  }
}
// wrap in an Option

fn main() {
  let vec = vec![1, 2];
  let fifth = take_fifth(vec);
  println!("{:?}", fifth);
  // .unwrap() - take out what is inside
  match fifth {
    Some(number) => println!("I got {}", number),
    None => println!("None..")
  }

  if fifth.is_some() {
    println!("wow I got {}", fifth.unwrap());
  }
  // unwrap from Option<i32> to i32
  // unwrap on None -> panic

  // .expect -> can make panic message
  fifth.expect("Needed at least five items - make sure Vec has at least 5 items");
}