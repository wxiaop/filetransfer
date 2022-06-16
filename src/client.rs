use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::io::{self, prelude::*, BufReader};
use std::net::UdpSocket;
const EXIT: &str = "exit()";
const PUT: &str = "put";
fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let ipaddr = &args[1].clone();
    let socket = UdpSocket::bind("0.0.0.0:9976")?;
    loop {
        socket.connect(ipaddr).expect("connection failed!");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read!!");
        let input: Vec<_> = input.trim_end().split_whitespace().collect();
        // 根据命令判断一下
        if input[0] == EXIT {
            // std::process::exit(1);
            return Ok(());
        } else if input[0] == PUT {
            let start = std::time::Instant::now();
            open_read_file(&socket, input[1]);
            let duration = start.elapsed();
            println!("执行时间为: {:?}", duration);
        }
    }
}

fn open_read_file(socket: &UdpSocket, filename: &str) {
    let mut w = [0; 49812];
    let file_s = fs::File::open(filename).expect("Failed to open file!!");
    socket.send(filename.as_bytes()).unwrap();
    let mut reader = BufReader::new(file_s);
    let mut count = 0;
    loop {
        let byte_s = reader.read(&mut w[..]).unwrap();
        count += 1;
        println!("{}", count);
        if byte_s != 0 {
            println!("==========> 正在上传 ...");
            socket.send(&w[..byte_s]).unwrap();
            // stream.flush().unwrap();
        } else {
            socket.send("UPLOAD_DONE".as_bytes()).unwrap();
            println!("=========== 上传完成 ===========");
            let mut buffer = [0; 2];
            socket.recv_from(&mut buffer).unwrap();
            let res = String::from_utf8(buffer.to_vec()).unwrap() == "OK";
            let res2 = buffer == "OK".as_bytes();
            let res3 = "OK" == String::from_utf8_lossy(&buffer);
            println!("recv : {:?}", buffer);
            println!("recv : {}", String::from_utf8(buffer.to_vec()).unwrap());
            println!("recv : {}", res);
            println!("recv : {}", res2);
            println!("recv : {}", res3);
            if res {
                println!("coming recv : {}", String::from_utf8_lossy(&buffer));
                break;
            }
        }
    }
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Filedata {
    id: i32,
    byte: Vec<u8>,
}
///struct => [u8]
pub unsafe fn struct_to_u8<T: Sized>(src: &T) -> &[u8] {
    ::std::slice::from_raw_parts((src as *const T) as *const u8, ::std::mem::size_of::<T>())
}
/// [u8] => struct
pub unsafe fn u8_to_struct(src: Vec<u8>) -> Filedata {
    std::ptr::read(src.as_ptr() as *const _)
}
