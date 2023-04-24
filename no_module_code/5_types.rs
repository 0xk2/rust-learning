#![allow(overflowing_literals)]

type NanoSecond = u64;

fn main() {
  let decimal = 65.4321_f32;

  // Error! No implicit conversion
  // let integer:u8 = decimal;

  let integer = decimal as u8;
  let character = integer as char;

  println!("Casting {} -> {} -> {}", decimal, integer, character);
  
  // when casting any value to an unsigned type, T,
  // T::MAX + 1 is added or subtracted until the value
  // fits into the new type

  // 1000 already fits in a u16
  println!("1000 as u16 is {}", 1000 as u16);

  // the first 8 least significant bits (LSB) are kept,
  // while the rest towards the most significant bit (MSB) get truncated
  println!("1000 as u8 is {}", 1000 as u8);

  println!("size of `decimal` {} in bytes: {}",decimal, std::mem::size_of_val(&decimal));

  // error if no type defined or nothing is pushed into vec
  let mut vec = Vec::new();
  vec.push(decimal);

  println!("{:?}", vec);

  let nano_seconds : NanoSecond = 50000;
  println!("nano seconds: {}",nano_seconds);
}