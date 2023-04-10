fn main() {
  
  // // Variables can be type annotated
  // let logical: bool = true;
  
  // let a_float: f64 = 1.0; // Regular annotation
  // let an_integer = 5i32; // Suffix annotation
  // // Or a default will be used
  // let default_float = 3.0; // `f64`
  // let default_integer = 7; // `i32`
  // // A type can also be inferred from context.
  // let mut inferred_type = 12; // Type i64 is inferred from another line.
  // inferred_type = 4294967296i64;

  // let mut mutable = 12; // Mutable `i32`
  // mutable = 21;

  // // Error! The type of a variable can't be changed
  // // mutable = true;

  // // Variables can be overwritten with shadowing
  // let mutable = true;
  let k = 0;
  println!("{:02X}",k);
  println!("0011 AND 0101 is {:b}", 0b0011 & 0b0101);
  let x = 0b0011;
  let y = 0b0101;
  println!("0011 OR 0101 is {:04b}", x | y);
  println!("1 << 5 is {:b}",1u32<<5);
  println!("1 << 5 is {}",1u32<<5);
  println!("One million is written as {}",1_000_000u32);
}