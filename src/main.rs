use gossip::whisper::Message;
use gossip::{self, neighborhood};
use std::net::ToSocketAddrs;
use std::sync::mpsc;
use std::thread;

fn client(rx: mpsc::Receiver<Message>, tx: mpsc::Sender<Message>) {
    let read_thread = thread::spawn(move || loop {
        let mut line = String::new();
        std::io::stdin()
            .read_line(&mut line)
            .expect("Failed to read stdin");
        let new_msg = Message::new(
            gossip::whisper::MessageType::Text,
            &neighborhood::Node::with_address(
                String::from("Zeus"),
                0,
                std::net::SocketAddr::new("0.0.0.0".parse().unwrap(), 0),
            ),
            &line,
            vec![0],
            0,
            &vec![0; 12],
            std::time::SystemTime::now(),
        );
        tx.send(new_msg)
            .expect("Unable to communicate with server!");
    });

    loop {
        match rx.try_recv() {
            Ok(msg) => {
                println!("{}", msg.format());
            }
            Err(mpsc::TryRecvError::Empty) => {
                thread::sleep(std::time::Duration::from_millis(200));
            }
            Err(mpsc::TryRecvError::Disconnected) => {
                break;
            }
        };
    }
    read_thread.join().expect("Unable to join threads!");
}

fn main() {
    let init_node_list: Vec<String> = std::env::args().collect();
    let mut connect_node = Vec::<std::net::SocketAddr>::new();
    if init_node_list.len() > 1 {
        for i in &init_node_list[1..] {
            let mut uri = i
                .to_socket_addrs()
                .expect("Unable to parse initial connections URI's");
            let addr = uri.next();
            let result = addr.unwrap();
            connect_node.push(result);
        }
    }
    let config = gossip::config::Config {
        max_send_peers: 5,
        stored_messages_filename: String::from("gossip.db"),
    };
    let (config_tx, config_rx) = mpsc::channel();
    let (client_tx, client_rx) =
        gossip::spawn_server(String::from("Aphrodite"), connect_node, config_rx);
    config_tx.send(config);
    let client_thread = thread::spawn(|| client(client_rx, client_tx));
    client_thread.join().expect("Unable to join threads");
}
