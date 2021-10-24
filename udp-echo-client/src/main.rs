use std::net::UdpSocket;
use std::io;
use std::str::from_utf8;
use clap::Parser;

// 限制接收最大缓存
const BUF_SIZE:usize = 512;

/// Simplest udp echo client example
#[derive(Parser, Debug)]
#[clap(name = "udp-echo-client")]
struct UdpEchoClient {
    /// IPv4 address
    #[clap(short, long, default_value = "127.0.0.1")]
    addr: String,

    /// IPv4 port
    #[clap(short, long, default_value = "9876")]
    port: u16,
}

fn main() {
    // 解析命令行参数
    let client = UdpEchoClient::parse();
    println!("UDP echo client connecting to {}:{}", client.addr, client.port);
    // 设置 UDP 客户端接收服务端 Response 的端口， 请求数据时会将接收端口号一起发送给服务端
    // 与 TCP 客户端不同， TCP 是面向连接的可靠通信，TCP 客户端连接 TCP 服务端时，Stream 会自动选择随机的端口，与服务端保持稳定的连接
    let socket = UdpSocket::bind(&(client.addr.to_string(), client.port-1)).expect("Cloud not bind client!");

    // UDP 客户端连接的 server 的地址
    socket
        .connect(&(client.addr.to_string(), client.port))
        .expect("Cloud not connect to server");

    loop {
        let mut input = String::new();
        let mut buffer = [0_u8; BUF_SIZE];

        println!("Type your input, or type 'exit'");

        io::stdin().read_line(&mut input).expect("Failed to read");

        if input.trim() == "exit" {
            println!("Exited!");
            break;
        }

        // 发送数据到 UDP 服务端
        socket
            .send(input.as_bytes())
            .expect("Failed to send to server");

        // 接收 UDP 服务端数据
        socket
            .recv_from(&mut buffer)
            .expect("Could not read into buffer");
        print!("{}",from_utf8(&buffer).expect("could write buffer as string"));
    }
}