use simple_logger;
use log::{set_max_level, info, LevelFilter};
use serde::{Deserialize, Serialize};
use std::{time::Duration, thread,net::{UdpSocket,SocketAddr}};

#[derive(Serialize, Deserialize, Debug)]
struct Message{
    code: u8,
    message: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Response{
    code: u8,
}

fn handle_message(buf: Vec<u8>, amt: usize, src: SocketAddr, socket: UdpSocket) -> std::io::Result<()> {
    let decode = String::from_utf8(buf[..amt].to_vec()).unwrap();

    let deserialized: Message = serde_json::from_str(&decode).unwrap();
    info!("recieved {}: {} [{}]", src, deserialized.message, deserialized.code);

    let response = Response { code: 200 };
    let response = serde_json::to_string(&response).unwrap();
    let response = response.as_bytes();

    // FIX: i think this is causing a blockage
    // let _bytes = socket.send_to(response, src)?;

    // might fix blockage
    loop {
        match socket.send_to(response, src) {
            Ok(bytes) => {
                info!("Sent {bytes} bytes from {src}");
                break;
            },
            Err(_) => {
                continue;
            }
        }
    }

    Ok(())
}

// NOTE: this can be spead up with a thread pool, won't have overhead of thread creation
fn main() -> std::io::Result<()> {
    simple_logger::init().unwrap();
    set_max_level(LevelFilter::Info);

    info!("Attempting Start Up");

    let socket = UdpSocket::bind("127.0.0.1:8000")?;
    socket.set_read_timeout(Some(Duration::from_millis(15))).unwrap();
    socket.set_write_timeout(Some(Duration::from_millis(15))).unwrap();
    // socket.set_nonblocking(true)?;

    info!("Listening");

    // TODO: create thread to handle closing handles, use channels to pass handles
    // let mut handles = Vec::new();

    loop {
        let mut buf = vec![0; 50];
        let res = socket.recv_from(&mut buf);

        let (amt, src) = match res {
            Ok(res) => (res.0, res.1),
            Err(_) => continue
        };
        
        let sock2 = socket.try_clone()?;

        let _handler = thread::spawn(move || -> std::io::Result<()>{
            return handle_message(buf, amt, src, sock2);
        });


        // handles.push(handler);
        // break;
    }

    /*
    #[warn(unreachable_code)]
    for i in handles {
        let _ = i.join().unwrap();
    }
    
    Ok(())
    */
}
