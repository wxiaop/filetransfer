use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, prelude::*, BufWriter};
use std::net::UdpSocket;
use std::thread;

fn main() -> io::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:9977")?;

    loop {
        let socket = socket.try_clone()?;
        let mut thread_list = vec![];
        let handle = thread::spawn(move || {
            receive_file(&socket).expect("fail");
        });
        thread_list.push(handle);

        for handle in thread_list {
            handle.join();
        }
    }
}

//接收文件
fn receive_file(socket: &UdpSocket) -> io::Result<()> {
    let mut buf = [0; 49812];
    // 第一次读取 读取文件名
    let (amt, src) = socket.recv_from(&mut buf)?;
    let filename = String::from_utf8_lossy(&buf[..amt]);
    // let filename: Vec<_> = filename.split(".").collect();
    // let namesub = format!("{}{}", timestamp(), filename);
    // println!("filename => {} ", filename.to_string());
    let file = fs::File::create(filename.to_string()).expect("Failed to create file!");
    let mut writer_buf = BufWriter::new(file);
    let mut count = 0;
    loop {
        let (amt, src) = socket.recv_from(&mut buf)?;
        println!("loop => {}", amt);
        count += 1;
        println!("times: => {}", count);

        // if amt == 49812 {
        //     println!("=========> 正在保存...");
        //     writer_buf.write_all(&buf[..amt])?;
        //     writer_buf.flush()?;
        // } else {
        //     println!("========= 保存完成 =========");
        //     writer_buf.write_all(&buf[..amt])?;
        //     writer_buf.flush()?;
        //     socket.send_to("OK".as_bytes(), &src)?;
        //     break;
        // }
        if "UPLOAD_DONE".as_bytes() == &buf[..amt] {
            println!("========= 保存完成 =========");
            socket.send_to("OK".as_bytes(), &src)?;
            break;
        } else {
            println!("=========> 正在保存...");
            writer_buf.write_all(&buf[..amt])?;
            writer_buf.flush()?;
        }
    }
    Ok(())
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Filedata {
    id: i32,
    byte: Vec<u8>,
}
// #[repr(C)]
// #[derive(Debug)]
// pub struct Filedata<'a> {
//     id: i32,
//     byte: &'a [u8],
// }
// ///struct => [u8]
// pub unsafe fn struct_to_u8<T: Sized>(src: &T) -> &[u8] {
//     ::std::slice::from_raw_parts((src as *const T) as *const u8, ::std::mem::size_of::<T>())
// }
// /// [u8] => struct
// pub unsafe fn u8_to_struct(src: Vec<u8>) -> Filedata<'static> {
//     std::ptr::read(src.as_ptr() as *const _)
// }
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
