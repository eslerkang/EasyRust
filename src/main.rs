// struct -> my own type / and
// enum / or

// unit struct
struct FileDirectory;
// tuple struct
#[derive(Debug)]
struct Color(u8, u8, u8);
// named struct
#[derive(Debug)]
struct Country {
  population: u32,
  capital: String,
  leader_name: String
}

fn takes_file_directory(_input: FileDirectory) {
  println!("haha");
}

fn main() {
  let x = FileDirectory;
  println!("{}", std::mem::size_of_val(&x));
  
  let color = Color(20, 20, 100);
  
  println!("{:?}", color);
  let korea = Country {
    population: 50_000_000,
    capital: String::from("seoul"),
    leader_name: String::from("Moon")
  };
  println!("{:#?}", korea);
}