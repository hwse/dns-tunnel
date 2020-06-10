use std::net::UdpSocket;
use dns_tunnel_lib::header::Header;

fn main() {
    let socket = UdpSocket::bind("0.0.0.0:5353").expect("Cant bind port");
    let message: [u8;5] = [1,2,3,4,5];
    let message_2: Vec<u8> = vec![5,1,1,1];
    let bytes_sent = socket.send_to(&message_2[..], "127.0.0.1:53").unwrap();
}
