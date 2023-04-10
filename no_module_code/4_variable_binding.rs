fn main() {
  let long_lived_binding = 1;
  let shadowed_binding = 1;
  // This is a block, and has a smaller scope than the main function
  {
    // This binding only exists in this block
    let short_lived_binding = 2;
    println!("inner short: {}", short_lived_binding);

    println!("before being shadowed: {}", shadowed_binding);
    // this binding *shadows* the outer one
    let shadowed_binding = "abc";
    println!("shadowed in inner block: {}", shadowed_binding);
  }
  // end of the block
  // println!("outer short: {}", short_lived_binding);
  println!("outer long: {}", long_lived_binding);

  let shadowed_binding = 1.2;
  println!("shadowed in outer block: {}",shadowed_binding);
}