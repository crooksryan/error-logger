mod structs;
use crate::structs::*;
use simple_logger;
use log::{set_max_level, info, LevelFilter};
use std::{time::Duration, net::{UdpSocket,SocketAddr}};

fn handle_message(buf: Vec<u8>, amt: usize, src: SocketAddr, socket: UdpSocket) -> std::io::Result<()> {
    let decode = String::from_utf8(buf[..amt].to_vec()).unwrap();

    let deserialized: Message = serde_json::from_str(&decode).unwrap();
    info!("recieved {}: {} [{}]", src, deserialized.message, deserialized.code);

    let response = Response { code: 200 };
    let response = serde_json::to_string(&response).unwrap();
    let response = response.as_bytes();

    let _bytes = socket.send_to(response, src)?;

    Ok(())
}

fn main() -> std::io::Result<()> {
    simple_logger::init().unwrap();
    set_max_level(LevelFilter::Info);

    info!("Start Up on port 8000");

    let socket = UdpSocket::bind("127.0.0.1:8000")?;
    socket.set_read_timeout(Some(Duration::from_millis(15))).unwrap();
    socket.set_write_timeout(Some(Duration::from_millis(15))).unwrap();
    socket.set_nonblocking(true)?;

    info!("Listening");

    loop {
        let mut buf = vec![0; 50];
        let res = socket.recv_from(&mut buf);

        let (amt, src) = match res {
            Ok(res) => (res.0, res.1),
            Err(_) => continue
        };
        
        let sock2 = socket.try_clone()?;

        handle_message(buf, amt, src, sock2)?;
    }
}
