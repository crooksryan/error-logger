package main

import (
	"encoding/json"
	"net"
	"os"
	"time"

	"github.com/charmbracelet/log"
)

var logger = log.NewWithOptions(os.Stdout, log.Options{
    ReportCaller: false,
    ReportTimestamp: true,
    TimeFormat: time.Kitchen,
    Prefix: " ",
});

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
        logger.Fatalf("Connection %s failed to read", conn.RemoteAddr())
    }

    var message Message
    err = json.Unmarshal(buf[:n], &message)

    if err != nil {
        logger.Printf("Connection %s failed to convert", conn.RemoteAddr())
        logger.Printf("Sending %s to simple", conn.RemoteAddr())

        packet := simple_pack{
            buf: buf,
            amt: n,
            conn: conn,
        }

        c <- packet
        return
    }

    logger.Printf("Recieved from %s: %s [%d]", conn.RemoteAddr(), message.Message, message.Code)

    // send response
    response := Response{Code: 200}
    res, err := json.Marshal(response)
    logger.Printf("Sending to %s", conn.RemoteAddr())
    conn.Write(res)
}

func simple_handle(listener <-chan simple_pack) {
    logger.Info("Simple thread running")

    for {
        pack := <- listener

        logger.Infof("Simple Rec from %s: %s", pack.conn.RemoteAddr(), string(pack.buf[:pack.amt]))
        res := []byte("mes rec")
        pack.conn.Write(res)

        go keep_listening(pack.conn)
    }
}

func keep_listening(conn net.Conn){
    for {
        buf := make([]byte, 1024)
        n, err := conn.Read(buf)

        if err != nil {
            logger.Warnf("Connection with %s ended\n", conn.RemoteAddr())
            return
        }

        logger.Infof("Read from %s: %s", conn.RemoteAddr(), string(buf[:n]))
    }
}

func main() {
    server, err := net.Listen("tcp", "127.0.0.1:8000")

    c := make(chan simple_pack)

    if err != nil {
        logger.Error("Couldn't listen on port")
        return
    }

    go simple_handle(c)

    logger.Infof("Listening on %s", server.Addr())

    for {
        conn, err := server.Accept()

        if err != nil {
            logger.Warn("bad connection")
            continue
        }

        go handle_message(conn, c)
    }
}
