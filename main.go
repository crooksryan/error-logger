package main

import (
    "log"
    "net"
    "encoding/json"
)

type simple_pack struct {
    buf  []byte
    amt  int
    conn net.Conn
}

type Message struct {
    Code    uint8   `json:"code"`
    Message string  `json:"message"`
}

type Response struct {
    Code uint8      `json:"code"`
}

func handle_message(conn net.Conn, c chan <- simple_pack) {
    buf := make([]byte, 1024)
    n, err := conn.Read(buf)
    
    if err != nil {
        log.Fatalf("Connection %s failed to read\n", conn.RemoteAddr())
    }

    var message Message
    err = json.Unmarshal(buf[:n], &message)

    if err != nil {
        log.Printf("Connection %s failed to convert\n", conn.RemoteAddr())
        log.Printf("Sending %s to simple", conn.RemoteAddr())

        packet := simple_pack{
            buf: buf,
            amt: n,
            conn: conn,
        }

        c <- packet
        return
    }

    log.Printf("Recieved from %s: %s [%d]\n", conn.RemoteAddr(), message.Message, message.Code)

    // send response
    response := Response{Code: 200}
    res, err := json.Marshal(response)
    log.Printf("Sending to %s\n", conn.RemoteAddr())
    conn.Write(res)
}

func simple_handle(listener <-chan simple_pack) {
    log.Println("Simple thread running")

    for {
        pack := <- listener

        log.Printf("Simple Rec from %s: %s\n", pack.conn.RemoteAddr(), string(pack.buf[:pack.amt]))
        res := []byte("mes rec")
        pack.conn.Write(res)
    }
}

func main() {
    /*
    udpAddr, err := net.ResolveUDPAddr("udp", "127.0.0.1:8000")
    server, err := net.ListenUDP("udp", udpAddr)
    */

    server, err := net.Listen("tcp", "127.0.0.1:8000")

    c := make(chan simple_pack)

    if err != nil {
        log.Println("Couldn't listen on port")
        return
    }

    go simple_handle(c)

    log.Printf("Listening on %s\n", server.Addr())
    for {
        conn, err := server.Accept()

        if err != nil {
            log.Println("bad connection")
            continue
        }

        go handle_message(conn, c)
    }
}
