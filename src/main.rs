mod structs;
use crate::structs::*;
use simple_logger;
use log::{set_max_level, info, LevelFilter};
use std::{time::Duration, net::{UdpSocket,SocketAddr}};

fn handle_message(buf: Vec<u8>, amt: usize, src: SocketAddr, socket: UdpSocket) -> std::io::Result<()> {
    let decode = String::from_utf8(buf[..amt].to_vec()).unwrap();

    let deserialized: Result<Message, serde_json::Error> = serde_json::from_str(&decode);

    if let Ok(deserialized) = deserialized {
        info!("recieved from {}: {} [{}]", src, deserialized.message, deserialized.code);

        let response = Response { code: 200 };
        let response = serde_json::to_string(&response).unwrap();
        let response = response.as_bytes();

        let _bytes = socket.send_to(response, src)?;

        Ok(())
    } else {
        Err(std::io::Error::new(std::io::ErrorKind::Other, "broke"))
    }
}

fn simple_handle(buf: Vec<u8>, amt: usize, src: SocketAddr, socket: UdpSocket) {
    let decode = String::from_utf8(buf[..amt].to_vec()).unwrap();

    info!("recieved simple from {}: {}", src, decode);

    let response = Response { code: 200 };
    let response = serde_json::to_string(&response);

    if let Ok(response) = response {
        let response = response.as_bytes();
        let _result_bytes = socket.send_to(response, src);
    } else {
        info!("couldn't encode res")
    }
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

        match handle_message(buf.clone(), amt, src, sock2) {
            Ok(_) => {},
            Err(_) => {
                info!("connection required simple handle");
                simple_handle(buf.clone(), amt, src, socket.try_clone()?);
            }
        }
    }
}
