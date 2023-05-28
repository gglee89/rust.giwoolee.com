fn main() {
  let message = "Hello World";
  let a = 10;
  let c = 15;
  println!("{}", print_welcome(message));
  println!("{}", add(a, c));
}

fn print_welcome(text: &str) -> String {
  return "Some text: ".to_owned() + text;
}

fn add(x: u32, y: u32) -> u32 {
  let sum = x + y;
  return sum
}