// slices
// vecs -> vector - next lecture
// &str

// Array Vec
// &str String
// String ==> Vec<u8>

// dynamically sized type

fn main() {
  let seasons = ["Spring", "Summer", "Fall", "Winter", "Spring", "Summer", "Fall", "Winter"];
  println!("{:?}", &seasons[..=4]);
}