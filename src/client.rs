use std::env;
use std::fs;
use std::io::{self, prelude::*, BufReader, BufWriter};
use std::net::TcpStream;
const EXIT: &str = "exit()";
const PUT: &str = "put";
fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let ipaddr = &args[1].clone();
    loop {
        let mut stream = TcpStream::connect(ipaddr)?;
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read!!");
        let input: Vec<_> = input.trim_end().split_whitespace().collect();
        // 根据命令判断一下
        if input[0] == EXIT {
            // std::process::exit(1);
            return Ok(());
        } else if input[0] == PUT {
            let start = std::time::Instant::now();
            open_read_file(&mut stream, input[1]);
            stream.shutdown(std::net::Shutdown::Both)?;
            let duration = start.elapsed();
            println!("执行时间为: {:?}", duration);
        }
    }

    // println!("w ==> {:#?}", w);
    // let start = std::time::Instant::now();
    //todo
    // let duration = start.elapsed();
    // println!("执行时间为: {:?}", duration);
    // Ok(())
}

fn open_read_file(stream: &mut TcpStream, filename: &str) {
    // let mut stream = BufWriter::new(stream);
    // let mut file_s = fs::File::open(filename).expect("Failed to open file!!");
    // println!("{:#?}", file_s);
    // stream.write(filename.as_bytes()).unwrap();
    // let mut w = [0; 49812];
    // loop {
    //     let byte_s = file_s.read(&mut w[..]).unwrap();
    //     if byte_s != 0 {
    //         println!("==========> 正在上传 ...");
    //         stream.write(&w[..byte_s]).unwrap();
    //         // stream.flush().unwrap();
    //     } else {
    //         println!("=========== 上传完成 ===========");
    //         break;
    //     }
    // }
    let mut w = [0; 49812];
    let file_s = fs::File::open(filename).expect("Failed to open file!!");
    stream.write(filename.as_bytes()).unwrap();
    let mut reader = BufReader::new(file_s);
    loop {
        let byte_s = reader.read(&mut w[..]).unwrap();
        if byte_s != 0 {
            println!("==========> 正在上传 ...");
            stream.write(&w[..byte_s]).unwrap();
            // stream.flush().unwrap();
        } else {
            println!("=========== 上传完成 ===========");
            break;
        }
    }
}
