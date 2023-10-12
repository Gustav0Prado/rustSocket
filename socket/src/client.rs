use std::{net::UdpSocket, io::Write, str};

pub fn udp_client(args: Vec<String>) {
   let socket = UdpSocket::bind(format!("127.0.0.1:0")).expect("couldn't bind to address");

   let mut buff = [0; 256];
   let mut test: &mut[u8] = &mut buff;
   test.write(args[4].as_bytes()).unwrap();


   socket.send_to(&buff, format!("{}:{}", args[2], args[3])).expect("Couldn't send message");

   println!("Sent msg: '{}'", str::from_utf8(&buff).unwrap());
}