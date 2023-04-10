use std::fmt;

// Tuples can be used as function arguments and as return variables
fn reverse(pair: (i32, bool)) -> (bool, i32) {
  // `let` can be used to bind the members of a tuple to variables
  let (int_param, bool_param) = pair;
  (bool_param, int_param)
}

struct Mtr2x2(f32, f32, f32, f32) ; // row1.1, row1.2, row2.1, row2.2

fn reverse_mtr(matrix:Mtr2x2) -> Mtr2x2 {
  Mtr2x2(matrix.0, matrix.2,matrix.1,matrix.3)
}

impl fmt::Display for Mtr2x2 {
  fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
    write!(f,"Matrix:")?;
    write!(f,"\n ({} {})", self.0, self.1)?;
    write!(f,"\n ({} {})", self.2, self.3)
  }
}

fn main() {
  let long_tuple = (1u8, 2u16, 3u32, 4u64, 'a');
  println!("Long tuple first value: {}", long_tuple.0);
  println!("Long tuple second value: {}", long_tuple.1);
  println!("tuple: {:?}", long_tuple);

  let tuple_of_tuples = ((1u8, 2u16), (4u32, true), "hello");

  println!("{:?}",tuple_of_tuples);
  let pair = (1, true);
  println!("reverse pair: {:?}",reverse(pair));
  let m = Mtr2x2(1.1, 1.2, 2.1, 2.2);
  println!("{}",m);
  println!("{}",reverse_mtr(m));
}