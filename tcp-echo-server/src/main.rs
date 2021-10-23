// 参考文档 https://doc.rust-lang.org/std/net/struct.TcpListener.html
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Write, Read};
use std::thread;
use std::str::from_utf8;
use clap::Parser;

// 限制接收最大缓存
const BUF_SIZE:usize = 512;

/// Simplest tcp echo server example
#[derive(Parser, Debug)]
#[clap(name = "tcp-echo-server")]
struct TcpEchoServer {
    /// IPv4 address
    #[clap(short, long, default_value = "127.0.0.1")]
    addr: String,

    /// IPv4 port
    #[clap(short, long, default_value = "9876")]
    port: u16,
}

fn handle_client(mut stream: TcpStream) {
    println!("Incoming connection from {}", stream.peer_addr().unwrap());

    let mut buf = [0_u8; BUF_SIZE];
    while match stream.read(&mut buf) {
        Ok(size) => {
            // byte 转字符串
            let data = from_utf8(&buf[0..size]).unwrap();
            if size > 0 {
                // echo back
                println!("Recv from client:{:?}", data);
                stream.write_all(&buf[0..size]).unwrap();
                true
            } else {
                // 消息长度为 0，关闭连接
                println!("Client connection closed: {}", stream.peer_addr().unwrap());
                // 确保关闭连接前，都写入
                // stream.flush().unwrap();
                // stream.shutdown(Shutdown::Both).unwrap();
                false
            }
        },
        Err(err) => {
            eprintln!("read stream from {} failed, err: {}", stream.peer_addr().unwrap(), err);
            // 确保关闭连接前，都写入
            stream.flush().unwrap();
            // 关闭连接避免占用资源，Both 表示读写都关闭
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    }{}
}

fn main() {
    // 解析命令行参数
    let server = TcpEchoServer::parse();
    println!("TCP echo server listening at {}:{}", server.addr, server.port);
    // 创建 socket，绑定端口
    let listener = TcpListener::bind(&(server.addr, server.port)).unwrap();
    // 遍历监听到的 TcpStream 迭代器
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // 连接成功
                thread::spawn(move || {
                    handle_client(stream);
                });
            }
            Err(e) => {
                // 连接失败
                eprintln!("stream failed, err: {}", e);
            }
        }
    }
    // close the socket server
    drop(listener);
}