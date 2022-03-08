use std::mem::size_of_val;

struct numbers {
  one: u8, // 1
  two: u8, // 1
  three: u8, // 1
  four: u32 // 4
  // 4 + 4 -> two 4bytes
}

#[derive(Debug)]
struct Country {
  population: u32,
  capital: String,
  leader: String,
  all_populations: [u32; 5500]
}
  
fn main() {
  let population = 50_000_000;
  let capital = "Seoul".to_string();
  let leader = "Moon".to_string();
  let country = Country {
    population,
    capital,
    leader,
    all_populations: [500; 5500]
  };
  println!("{:#?}", country);
  println!("size of country: {}", size_of_val(&country));

  let number = numbers{
    one: 1,
    two: 2,
    three: 3,
    four: 32
  };

  println!("size of number: {}", size_of_val(&number));
}