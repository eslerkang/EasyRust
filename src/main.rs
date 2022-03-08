#[derive(Debug)]
struct Country {
  population: u32,
  capital: String,
  leader: String
}
  
fn main() {
  let population = 50_000_000;
  let capital = "Seoul".to_string();
  let leader = "Moon".to_string();
  let country = Country {
    population,
    capital,
    leader
  };
  println!("{:#?}", country)
}