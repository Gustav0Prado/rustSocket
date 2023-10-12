use std::{net::UdpSocket, io::Write, str};

fn server(args: Vec<String>) {
    println!("Iniciado como servidor na porta {}\n", args[2]);

    loop {
        let socket = UdpSocket::bind(format!("127.0.0.1:{}", args[2])).expect("couldn't bind to address");

        let mut buf = [0; 256];
        let (number_of_bytes, src_addr) = socket.recv_from(&mut buf).expect("Didn't receive data");

        println!("Recebeu!\nmsg: {}\nsize: {}\nsource:{}", str::from_utf8(&buf).unwrap(), number_of_bytes, src_addr);
    }
}

fn client(args: Vec<String>) {
    let socket = UdpSocket::bind(format!("127.0.0.1:0")).expect("couldn't bind to address");

    let mut buff = [0; 256];
    let mut test: &mut[u8] = &mut buff;
    test.write(args[4].as_bytes()).unwrap();


    socket.send_to(&buff, format!("{}:{}", args[2], args[3])).expect("Couldn't send message");

    println!("Enviou msg: '{}'", str::from_utf8(&buff).unwrap());
}

fn help() {
    println!("Usos corretos:\n");
    println!("\t./main servidor <porta>\n");
    println!("\t./main cliente <servidor> <porta> <mensagem>:\n");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    match args[1].as_str() {
        "cliente" => client(args),
        "servidor" => server(args),
        _ => help(),
    }
}