mod structs;
use crate::structs::*;
use simple_logger;
use std::net::SocketAddr;
use log::{
    set_max_level,
    info,
    LevelFilter
};
use tokio::{
    io::{
        AsyncReadExt,
        AsyncWriteExt
    }, 
    net::{
        TcpListener,
        TcpStream
    }
};

async fn handle_message(stream: &mut TcpStream, src: &SocketAddr) -> std::io::Result<()> {
    let mut buf = String::new();
    let _n = stream.read_to_string(&mut buf).await?;

    let deserialized: Result<Message, serde_json::Error> = serde_json::from_str(&buf);

    if let Ok(deserialized) = deserialized {
        info!("recieved from {}: {} [{}]", src, deserialized.message, deserialized.code);

        let response = Response { code: 200 };
        let response = serde_json::to_string(&response).unwrap();
        let response = response.as_bytes();

        // let _bytes = socket.send_to(response, src)?;
        stream.write(response).await?;

        Ok(())
    } else {
        info!("Src {}: couldn't deserialize", src);
        Err(std::io::Error::new(std::io::ErrorKind::Other, "broke"))
    }
}

async fn simple_handle(stream: &mut TcpStream, src: &SocketAddr) -> std::io::Result<()> {
    let mut buf = String::new();
    let _n = stream.read_to_string(&mut buf).await?;

    info!("recieved simple from {}: {}", src, buf);

    let response = Response { code: 200 };
    let response = serde_json::to_string(&response);

    if let Ok(response) = response {
        let response = response.as_bytes();
        let _result_bytes = stream.write(response).await?;
    } else {
        info!("couldn't encode res")
    }

    Ok(())
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    simple_logger::init().unwrap();
    set_max_level(LevelFilter::Info);

    info!("Start Up on port 8000");

    let listener = TcpListener::bind("127.0.0.1:8000").await?;

    info!("Listening...");

    loop {
        if let Ok((mut stream, addr)) = listener.accept().await {
            tokio::spawn(async move {
                let mut structured = 0;
                loop {
                    if structured == 1 {
                        let res = handle_message(&mut stream, &addr).await;

                        if let Err(_) = res {
                            structured = 1
                        }
                    } else {
                        let res = simple_handle(&mut stream, &addr).await;

                        if let Err(_) = res {
                            info!("Ending connection with: {}", addr);
                            break;
                        }
                    }
                }
            });


        } else {
            info!("Error when accepting connection")
        }
    }

}
