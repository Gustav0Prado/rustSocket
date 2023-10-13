pub fn help(err_type: &str) {
   println!("Correct usage:\n");

   match err_type {
      "client" => println!("\t./client <protocol> <server> <port> '<message>':\n"),
      "server" => println!("\t./server <protocol> <port>\n"),
      _ => println!("\tError\n"),
   }

   std::process::exit(1);
}