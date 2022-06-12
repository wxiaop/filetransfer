use std::env;
use std::fs;
use std::io::{self, prelude::*, BufWriter};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn main() -> io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:9977")?;
    // let mut thread_list: Vec<thread::JoinHandle<()>> = Vec::new();

    let mut thread_list = vec![];
    for stream in listener.incoming() {
        let stream = stream.expect("failed");
        let handle = thread::spawn(move || {
            receive_file(&stream);
        });
        thread_list.push(handle);
    }
    for handle in thread_list {
        handle.join();
    }
    Ok(())
}

//接收文件
fn receive_file(mut stream: &TcpStream) -> io::Result<()> {
    let mut buf = [0; 49812];
    // 第一次读取 读取文件名
    let byte_read = stream.read(&mut buf)?;

    let filename = String::from_utf8_lossy(&buf[..byte_read]);

    // let filename: Vec<_> = filename.split(".").collect();
    // let namesub = format!("{}{}", timestamp(), filename);
    println!("filename => {} ", filename.to_string());
    let file = fs::File::create(filename.to_string()).expect("Failed to create file!");
    let mut writer_buf = BufWriter::new(file);
    loop {
        let byte_read = stream.read(&mut buf)?;
        if byte_read != 0 {
            println!("=========> 正在保存...");
            writer_buf.write_all(&buf[..byte_read])?;
            writer_buf.flush()?;
        } else {
            println!("========= 保存完成 =========");
            break;
        }
    }
    Ok(())
}
///获取时间戳
fn _timestamp() -> i64 {
    let start = std::time::SystemTime::now();
    let since_the_epoch = start
        .duration_since(std::time::UNIX_EPOCH)
        .expect("Time went backwards");
    let ms = since_the_epoch.as_secs() as i64 * 1000i64
        + (since_the_epoch.subsec_nanos() as f64 / 1_000_000.0) as i64;
    ms
}
