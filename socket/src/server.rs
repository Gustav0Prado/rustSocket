use std::{net::UdpSocket, str};
mod utils;

fn udp_server(args: Vec<String>) {
   println!("Started as server on port: {}\n", args[2]);

   loop {
       let socket = UdpSocket::bind(format!("127.0.0.1:{}", args[2])).expect("couldn't bind to address");

       let mut buf = [0; 256];
       let (number_of_bytes, src_addr) = socket.recv_from(&mut buf).expect("Didn't receive data");

       println!("Received!\nmsg: {}\nsize: {}\nsource:{}", str::from_utf8(&buf).unwrap(), number_of_bytes, src_addr);
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() != 3 { utils::help("server")  }

    match args[1].as_str() {
        //"tcp" => tcp_server(args),
        "udp" => udp_server(args),
        _ => utils::help("server"),
    }
}