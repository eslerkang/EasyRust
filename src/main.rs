// tuples - multiple type in one / struct
// Vec<String>

// Destructuring
// Structure

fn main() {
  let tup = (3, "sdf", vec![3, 4, 5], String::from("asdfasd")); // 3 in this tup is i32
  println!("{:?}", tup);

  // array[0], vec[3]...
  println!("{}", tup.3);
  println!("{:?}", tup.2.get(1));
  
  // we can use Vec<(String, i32)>
  let vec_tup = vec![(String::from("asdf"), 21), (String::from("hey"), 1)];
  println!("{:?}", vec_tup);

  let str_tup = ("1", "2", "3445");
  let (a, b, _) = str_tup;
  println!("{}, {}", a, b);
  
  let str_arr = ["1", "2", "3445"];
  let [a, b, _] = str_arr;
  println!("{}, {}", a, b);
}