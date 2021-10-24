use std::net::UdpSocket;
use std::thread;
use std::str::from_utf8;
use clap::Parser;

// 限制接收最大缓存
const BUF_SIZE:usize = 512;

/// Simplest udp echo server example
#[derive(Parser, Debug)]
#[clap(name = "udp-echo-server")]
struct UdpEchoServer {
    /// IPv4 address
    #[clap(short, long, default_value = "127.0.0.1")]
    addr: String,

    /// IPv4 port
    #[clap(short, long, default_value = "9876")]
    port: u16,
}

fn main() {
    // 解析命令行参数
    let server = UdpEchoServer::parse();
    println!("UDP echo server listening at {}:{}", server.addr, server.port);
    // 创建 socket，绑定端口
    let socket = UdpSocket::bind(&(server.addr, server.port)).unwrap();
    loop {
        let mut buf = [0; BUF_SIZE];

        // 克隆 socket，闭包中需要 move，转移所有权
        let sock = socket.try_clone().expect("Failed to Clone socket()");
        match socket.recv_from(&mut buf) {
            Ok((size, src)) => {
                thread::spawn(move || {
                    println!("Handing connection from {}", src);
                    let data = from_utf8(&buf[..size]).unwrap();
                    println!("Recv from client:{:?}", data);
                    // UDP 不是面向连接的， 发送数据时，需要指定接收端地址，
                    sock.send_to(&buf[..size], &src)
                        .expect("Failed to send response");
                });
            }
            Err(e) => {
                eprintln!("Could receive a message: {}", e);
            }
        }
    }
}