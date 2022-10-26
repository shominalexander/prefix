fn main() {
 loop {
  let mut input : String = String::new();
  let mut prefix: String = String::new();

  println!("\r\n\r\nwords:");

  std::io::stdin().read_line(&mut input).expect("Input failed");

  input = input.replace("\n", "");
  input = input.replace("\r", "");

  if &input[..] == "exit" {
   break;   

  } else {//if &input[..] == "exit" {
   let text: Vec<String> = serde_json::from_str(&input[..]).expect("Wrong format");
   let last: usize       = text.len()                                             ;

   if last > 0usize {
    if last == 1 {
     prefix = text[0].clone();

    } else {//if last == 1 {
     let     first  : Vec<char> = text[0].chars().collect();
     let mut minimum: usize     = first.len()              ;

     for i in 1..last {
      let next  : Vec<char> = text[i].chars().collect();
      let length: usize     = next.len()               ;

      if length < minimum {
       minimum = length;

      }//if length < minimum {

      for j in 0..minimum {
       if next[j] != first[j] {
        if j < minimum {
         minimum = j;

        }//if j < minimum {

        break;
       }//if next[j] != first[j] {
      }//for j in 0..minimum {
     }//for i in 1..last {

     if minimum > 0usize {
      prefix = first[0..minimum].iter().collect();

     }//if minimum > 0usize {
    }//} else {//if last == 1 {
   }//if last > 0usize {

   println!("\r\nprefix:\r\n{:?}", prefix);
  }//} else {//if &input[..] == "exit" {
 }//loop {
}//fn main() {
