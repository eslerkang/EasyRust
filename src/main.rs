// collection types
// Array

// buffer

#[allow(unused_variables)]

fn main() {
  let array = ["One", "Two"]; // [&str; 2]
  let array2 = ["One", "Two", "Five"]; // [&str; 3] 
  let array3 = ["One", "Twoo"];
  // these two arrays are different type
  println!("Is array the same as array2? {}", array == array3);

  let array4 = [0; 640];
  println!("{:?}", array4);

  let month = ["1", "2"];
  println!("{:?}", month.get(1));
  // Option enum
  println!("{:?}", month.get(3));
}