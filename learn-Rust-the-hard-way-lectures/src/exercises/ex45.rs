use std::env;
use std::io::prelude::*;
use std::net::{Ipv4Addr, SocketAddrV4, TcpStream};

fn main() -> std::io::Result<()> {
    let mut args = Vec::new();
    for arg in env::args() {
        args.push(arg);
    }
    assert_ne!(args.len(), 3, "USAGE: netclient host port\n");
    //learncodethehardway.org:80 todo：直接使用域名来进行socket连接
    //let addr1 = SocketAddrV4::new(Ipv4Addr::from(""), 80);
    let addr = SocketAddrV4::new(Ipv4Addr::new(95, 216, 45, 211), 80);

    if let Ok(mut stream) = TcpStream::connect(addr) {
        println!("Connected to the server!");
        stream.write(&[1, 2]).expect("Fail to write in socket.");
        let mut buf = String::new();
        stream
            .read_to_string(&mut buf)
            .expect("Fail to read in socket.");
        println!("{}", buf);
    } else {
        println!("Couldn't connect to server...");
    }

    Ok(())
}
