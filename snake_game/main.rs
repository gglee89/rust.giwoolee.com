fn main() {
  let message = String::from("Hello");
  let message_2 = &message;
  // message_2 is not owner of message
  // message_2 is borrowing a reference to message

  println!("{}", message);
  println!("{}", message_2);
}