use std::io::{self, BufRead, BufReader, Write};
use std::net::{TcpStream, Shutdown};
use std::str::from_utf8;
use clap::Parser;

/// Simplest tcp echo client example
#[derive(Parser, Debug)]
#[clap(name = "tcp-echo-client")]
struct TcpEchoClient {
    /// IPv4 address
    #[clap(short, long, default_value = "127.0.0.1")]
    addr: String,

    /// IPv4 port
    #[clap(short, long, default_value = "9876")]
    port: u16,
}

fn main() {
    // 解析命令行参数
    let client = TcpEchoClient::parse();
    println!("TCP echo client connet to {}:{}", client.addr, client.port);
    // 建立 TCP 长连接
    let mut stream = TcpStream::connect(&(client.addr, client.port)).expect("Could not connect to server");
    loop {
        let mut input = String::new();
        let mut buffer: Vec<u8> = Vec::new();

        println!("Type your input, or type 'exit'");

        // 使用 read_line 从标准输入读取数据
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read from stdin");

        if input.trim() == "exit" {
            println!("Exited!");
            break;
        }

        // 将数据发送到服务端
        stream
            .write_all(input.as_bytes())
            .expect("Failed to write to server");

        // 使用 BufReader 包装 stream
        let mut reader = BufReader::new(&stream);

        // reader.read_to_end(&mut buffer).expect("Failed to read"); // 读取到 eof，需要 server 断开才能读完。
        // 读取缓存区的数据，因为连接不会断开，需要检测读到换行符后打印。
        reader
            .read_until(b'\n', &mut buffer)
            .expect("Could not read into buffer");
        
        // 将读取的数据输出
        print!("{}",from_utf8(&buffer).expect("Could not write buffer as string"));
    }
    // 确保关闭连接前，都写入
    stream.flush().unwrap();
    // 关闭连接避免占用资源，Both 表示读写都关闭
    stream.shutdown(Shutdown::Both).unwrap();
}