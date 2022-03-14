// .
// .

struct Item {
  num: u8
}

// .dot operator
impl Item {
  fn compare_number(&self, other_num: u8) {
    println!("{}", self.num == other_num);
  }
}

// Deref = *
fn main() {
  let item = Item {
    num: 10
  };
  let refer = &item.num; // u8
  let refer_item = &item; // &Item
  let refrefer_item = &refer_item; // &&Item -> **
  item.compare_number(10);
  refer_item.compare_number(10);
  refrefer_item.compare_number(10);

  println!("Are they the same? {}", *refer == 10u8);
}