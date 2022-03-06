// Vec - Vector
// Array = [u8; 10];
// Vec = Vec<String>

// T = some type
// generics

#[allow(unused_variables)]

fn main() {
  let name1 = String::from("Windy");
  let name2 = String::from("Gomesy");

  let mut vec = Vec::new();
  println!("Space for my vec: {}", vec.capacity());
  vec.push(name1);
  println!("Space for my vec: {}", vec.capacity());
  vec.push(name2);
  // let vec = vec![name1, name2];
  println!("Space for my vec: {}", vec.capacity());
  println!("{:?}", vec);
}