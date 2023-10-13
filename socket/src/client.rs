use std::{net::UdpSocket, io::Write, str};
mod utils;

fn udp_client(args: Vec<String>) {
   let socket = UdpSocket::bind(format!("127.0.0.1:0")).expect("couldn't bind to address");

   let mut buff = [0; 256];
   let mut test: &mut[u8] = &mut buff;
   test.write(args[4].as_bytes()).unwrap();


   socket.send_to(&buff, format!("{}:{}", args[2], args[3])).expect("Couldn't send message");

   println!("Sent msg: '{}'", str::from_utf8(&buff).unwrap());
}

fn main() {
   let args: Vec<String> = std::env::args().collect();
   
   if args.len() != 5 { utils::help("client") }

   match args[1].as_str() {
      //"tcp" => tcp_server(args),
      "udp" => udp_client(args),
      _ => utils::help("client"),
   }
}