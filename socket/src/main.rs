mod client;
mod server;

fn help() {
    println!("Correct usage:\n");
    println!("\t./main server <port>\n");
    println!("\t./main client <server> <port> '<message>':\n");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    match args[1].as_str() {
        "client" => client::udp_client(args),
        "server" => server::udp_server(args),
        _ => help(),
    }
}