use env_logger;
use gossip::whisper::{Message, MessageType};
use gossip::{self, neighborhood};
use std::net::ToSocketAddrs;
use std::sync::mpsc;
use std::thread;

fn client(client_name: String, mut client_handle: gossip::config::ClientHandle) {
    let mut sender_handle = client_handle.clone();
    let read_thread = thread::spawn(move || loop {
        let mut line = String::new();
        std::io::stdin()
            .read_line(&mut line)
            .expect("Failed to read stdin");
        let new_msg = Message::from_client(MessageType::Text, &client_name, &line);
        sender_handle.send_msg(new_msg);
    });

    loop {
        match client_handle.get_msg() {
            Ok(msg) => {
                println!("{}", msg.format());
            }
            Err(_) => {
                println!("Error happened!");
            }
        };
    }
}

fn main() {
    env_logger::init();
    let init_node_list: Vec<String> = std::env::args().collect();
    let mut client_name = None;
    let mut connect_node = Vec::<std::net::SocketAddr>::new();
    if init_node_list.len() > 1 {
        client_name = Some(init_node_list[1].to_string());
        for i in &init_node_list[2..] {
            let mut uri = i
                .to_socket_addrs()
                .expect("Unable to parse initial connections URI's");
            let addr = uri.next();
            let result = addr.unwrap();
            connect_node.push(result);
        }
    }
    let client_name = client_name.expect("No name given!");
    let config = gossip::config::Config {
        max_send_peers: 5,
        stored_messages_filename: String::from("gossip.db"),
    };
    let mut client_handle = gossip::spawn_server(client_name.clone(), connect_node);
    client_handle.update_config(config);
    let client_thread = thread::spawn(move || client(client_name, client_handle));
    client_thread.join().expect("Unable to join threads");
}
