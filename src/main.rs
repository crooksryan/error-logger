use simple_logger;
use log::{
    set_max_level,
    info,
    LevelFilter,
};
use std::net::UdpSocket;
use serde::{Deserialize, Serialize};
// use serde_json::json;


#[derive(Serialize, Deserialize, Debug)]
struct Message{
    code: u8,
    message: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Response{
    code: u8,
}

fn main() -> std::io::Result<()> {
    simple_logger::init().unwrap();
    set_max_level(LevelFilter::Info);

    info!("Attempting Start Up");

    let socket = UdpSocket::bind("127.0.0.1:8000")?;
    info!("Listening");

    loop {
        let mut buf = vec![0; 50];
        let (amt, src) = socket.recv_from(&mut buf)?;

        let decode = String::from_utf8(buf[..amt].to_vec()).unwrap();
        println!("Unserialized: {decode}");

        let deserialized: Message = serde_json::from_str(&decode).unwrap();

        println!("Recieved: {:?}", deserialized);

        let response = Response { code: 200 };
        let response = serde_json::to_string(&response).unwrap();
        let response = response.as_bytes();

        let bytes = socket.send_to(response, src)?;
        println!("Sent: {}", bytes);
    }
}
